/**
 * Copyright (C) 2022 Romain CADILHAC
 *
 * This file is part of Silicium.
 *
 * Silicium is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Silicium is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Silicium. If not, see <http://www.gnu.org/licenses/>.
 */
#include <core/elf.h>
#include <core/module.h>
#include <core/symbol.h>
#include <mm/malloc.h>
#include <mm/vmalloc.h>
#include <lib/string.h>
#include <lib/spinlock.h>

#define MODULE_INVALID_SYMBOL 0xFFFFFFFF

static DECLARE_LIST(module_list);
static DECLARE_SPINLOCK(lock);

static module_t *module_get(const char *name)
{
    spin_lock(&lock);
    list_foreach (&module_list, entry) {
        module_t *module = list_entry(entry, module_t, node);
        if (strcmp(module->name, name) == 0) {
            spin_unlock(&lock);
            return module;
        }
    }
    spin_unlock(&lock);
    return NULL;
}

/**
 * @brief 
 * 
 * @param ehdr 
 * @param symbtab 
 * @param idx 
 * @return int 
 */
static int module_elf_get_symbval(
    const elf_ehdr_t *ehdr,
    const elf_shdr_t *symbtab,
    const unsigned int idx)
{
    if (idx >= symbtab->size / symbtab->entsize)
        return -1;

    const vaddr_t symbaddr = (vaddr_t) ehdr + symbtab->offset;
    const elf_sym_t *symbol = (elf_sym_t *) (symbaddr + idx * symbtab->entsize);

    if (symbol->shndx == ELF_SHN_UNDEF) {
        // Undefined symbol
        const elf_shdr_t *strtab = elf_get_section(ehdr, symbtab->link);
        const char *name = (const char *) ehdr + strtab->offset + symbol->name;
        const vaddr_t value = symbol_get_value(name);
        if (value > 0)
            return value;
        if(ELF_ST_BIND(symbol->info) == ELF_STB_WEAK)
            return 0;
        
        error("module_load(): Unable to find symbol %s", name);
    } else if (symbol->shndx == ELF_SHN_ABS) {
        // Absolute symbol
        return (int) symbol->value;
    } else {
        // Internal symbol
        const elf_shdr_t *section = elf_get_section(ehdr, symbol->shndx);
		return (int) ehdr + section->offset + symbol->value;
    }

    return -1;
}

/**
 * @brief 
 * 
 * @param ehdr The ELF header
 * @param section The relocation section
 * @param relocation The relocation entry
 * @return int 
 */
static int module_elf_relocate_symbol(
    const elf_ehdr_t *ehdr,
    const elf_shdr_t *section,
    const elf_rel_t *relocation)
{
    const elf_shdr_t *target = elf_get_section(ehdr, section->info);
    uint32_t *base = 
        (uint32_t *) ((char *) ehdr + target->offset + relocation->offset);

    // Get the symbol value if needed
    uint32_t value = 0;
    if (ELF32_R_SYM(relocation->info) != ELF_SHN_UNDEF) {
        const elf_shdr_t *sym_section = 
            elf_get_section(ehdr, section->link);
        value = module_elf_get_symbval(
            ehdr,
            sym_section,
            ELF32_R_SYM(relocation->info));

        if (value == MODULE_INVALID_SYMBOL)
            return -ENOENT;
    }

    // Relocation type
    switch (ELF32_R_TYPE(relocation->info)) {
        case ELF_RTT_NONE:
            break;
        case ELF_RTT_32:
            *base = *base + value;
            break;
        case ELF_RTT_PC32:
            *base = *base + value - (uint32_t) base;
            break;
        default:
            trace("module_load(): Unknown relocation type %d", 
                ELF32_R_TYPE(relocation->info));
            return -EINVAL;
    }

    return 0;
}

static vaddr_t module_elf_find_symbol(
    const elf_ehdr_t *ehdr,
    const char *name,
    const uint8_t type,
    const uint8_t bind,
    const uint8_t visibility)
{
    const elf_shdr_t *shdr = (elf_shdr_t *) ((const char *)ehdr + ehdr->shoff);

    for (unsigned int i = 0; i < ehdr->shnum; i++) {
        const elf_shdr_t *section = &shdr[i];
        if (section->type != ELF_SHT_TYPE_SYMTAB)
            continue;

        const elf_shdr_t *strtab = elf_get_section(ehdr, section->link);
        const elf_sym_t *symbols = 
            (elf_sym_t *) ((const char *) ehdr + section->offset);
    
        for (unsigned int j = 0; j < elf_section_entry_count(section); j++) {
            const char *symbol_name = 
                (const char *) ehdr + strtab->offset + symbols[j].name;
            if (strcmp(symbol_name, name) == 0) {
                if (ELF_ST_BIND(symbols[j].info) != ELF_ST_BIND(bind))
                    continue;
                if (ELF_ST_TYPE(symbols[j].info) != ELF_ST_TYPE(type))
                    continue;
                if (symbols[j].other != visibility)
                    continue;
                // Return the symbol value
                return module_elf_get_symbval(
                    ehdr,
                    section,
                    j);
            }
        }
    }

    return ELF_INVALID_SYMBOL;
}

/**
 * @brief 
 * 
 * @param data 
 * @return int 
 */
static int module_elf_parse(char *data)
{
    elf_ehdr_t *ehdr = (elf_ehdr_t *) data;
    elf_shdr_t *shdr = (elf_shdr_t *) (data + ehdr->shoff);

    // TODO: Add more checks

    // Check if the elf file is valid
    if (ehdr->ident[ELF_IDENT_MAGIC0] != ELF_MAGIC0 ||
        ehdr->ident[ELF_IDENT_MAGIC1] != ELF_MAGIC1 ||
        ehdr->ident[ELF_IDENT_MAGIC2] != ELF_MAGIC2 ||
        ehdr->ident[ELF_IDENT_MAGIC3] != ELF_MAGIC3)
        return -EFAULT;
    // Check if the elf file is for x86 architecture
    if (ehdr->ident[ELF_IDENT_CLASS] != ELF_CLASS32)
        return -EFAULT;
    // Check if the data layout is little endian
    if (ehdr->ident[ELF_IDENT_DATA] != ELF_DATA_LSB)
        return -EFAULT;
    // Check if the elf file is relocatable
    if (ehdr->type != ELF_TYPE_REL)
        return -EFAULT;
    // Check if there is a string table
    if (ehdr->shstrndx == ELF_SHN_UNDEF)
        return -EFAULT;

    // Allocate bss if needed
    // FIXME: MEMORY LEAK if the module is unloaded
    for (unsigned int i = 0; i < ehdr->shnum; i++) {
        elf_shdr_t *section = &shdr[i];
        if (section->type == ELF_SHT_TYPE_NOBITS) {
            if (!(section->flags & ELF_SHT_ATTRIB_ALLOC))
                continue;
            if (section->size == 0)
                continue;
            const vaddr_t memory = vmalloc(
                section->size,
                VMALLOC_MAP | VMALLOC_ZERO);
            section->offset = (elf_off_t) (memory - (vaddr_t) ehdr);
        }
    }

    // Itenerate over the section and if it is a relocation section,
    // relocate the symbols
    int ret = 0;
    for (unsigned int i = 0; i < ehdr->shnum; i++) {
        elf_shdr_t *section = &shdr[i];
        if (section->type != ELF_SHT_TYPE_REL)
            continue;

        // Relocate symbols
        const unsigned int count = section->size / section->entsize;
        for (unsigned int j = 0; j < count; j++) {
            elf_rel_t *rel = &((elf_rel_t *) (data + section->offset))[j];
            if (module_elf_relocate_symbol(ehdr, section, rel) < 0)
                ret = -EFAULT;
        }
    }

    return ret;
}

/**
 * @brief Allocates a module structure and initialize the list node. All
 * other fields are set to NULL
 * 
 * @return module_t* The allocated module structure or
 *  NULL if an error occured.
 */
static module_t *module_allocate(void)
{
    module_t *module = malloc(sizeof(module_t));
    if (module == NULL)
        return NULL;

    list_init(&module->node);
    module->description = NULL;
    module->version = NULL;
    module->author = NULL;
    module->finit = NULL;
    module->init = NULL;
    module->name = NULL;
    module->elf = NULL;
    module->usage = 1;
    return module;
}

/**
 * @brief 
 * 
 * @param data 
 * @param name 
 * @return int 
 */
int module_load(char *data)
{
    module_t *module = module_allocate();
    if (module == NULL)
        return -ENOMEM;

    // Parse the ELF file
    const int ret = module_elf_parse(data);
    if (ret < 0) {
        free(module);
        return ret;
    }

    // TODO: Export module symbol
    vaddr_t mod_exit = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_exit__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);
    vaddr_t mod_init = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_init__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);
    vaddr_t mod_name = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_name__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);
    vaddr_t mod_author = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_author__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);
    vaddr_t mod_version = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_version__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);
    vaddr_t mod_description = module_elf_find_symbol(
        (elf_ehdr_t *) data,
        "__module_description__",
        ELF_STT_OBJECT,
        ELF_STB_LOCAL,
        ELF_STV_DEFAULT);

    if (mod_exit == ELF_INVALID_SYMBOL)
        mod_exit = 0;
    if (mod_init == ELF_INVALID_SYMBOL)
        mod_init = 0;
    if (mod_name == ELF_INVALID_SYMBOL)
        mod_name = 0;
    if (mod_author == ELF_INVALID_SYMBOL)
        mod_author = 0;
    if (mod_version == ELF_INVALID_SYMBOL)
        mod_version = 0;
    if (mod_description == ELF_INVALID_SYMBOL)
        mod_description = 0;

    // This in the only required field
    if (mod_name == 0) {
        error("Trying to load a kernel module without name");
        free(module);
        return -EFAULT;
    }

    module->elf = data;
    module->name = *(const char **) mod_name;
    if (module_exist(module->name)) {
        error("Module %s already loaded", module->name);
        free(module);
        return -EFAULT;
    }

    trace("Module %s loaded", module->name);
    if (mod_init != 0)  {
        module->init = *(module_init_t *) mod_init;
        trace("Module %s has a init function at 0x%p", 
            module->name, module->init);
    }
    if (mod_exit != 0) {
        module->finit = *(module_init_t *) mod_exit;
        trace("Module %s has a finit function at 0x%p", 
            module->name, module->finit);
    }
    if (mod_author != 0) {
        module->author = *(const char **) mod_author;
        trace("Module author: %s", module->author);
    }
    if (mod_version != 0) {
        module->version = *(const char **) mod_version;
        trace("Module version: %s", module->version);
    }
    if (mod_description != 0) {
        module->description = *(const char **) mod_description;
        trace("Module description: %s", module->description);
    }

    spin_lock(&lock);
    list_add(&module_list, &module->node);
    spin_unlock(&lock);

    if(module->init != NULL)
        module->init();
    return 0;
}

/**
 * @brief 
 * 
 * @param name 
 * @return int 
 */
int module_unload(const char *name)
{
    module_t *module = module_get(name);
    if (module == NULL)
        return -ENOENT;

    // Verify if we can unload the module
    if (module->usage > 1)
        return -EBUSY;

    spin_lock(&lock);
    list_remove(&module->node);
    spin_unlock(&lock);

    // TODO: Remove module's symbols from the symbol table
    if(module->finit != NULL)
        module->finit();
    free(module);
    return 0;
}

/**
 * @brief 
 * 
 * @param name 
 * @return int 
 */
int module_exist(const char *name)
{
    return module_get(name) != NULL;
}
