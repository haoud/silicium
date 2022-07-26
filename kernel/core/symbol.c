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
#include <lib/string.h>
#include <core/symbol.h>
#include <lib/spinlock.h>
#include <mm/malloc.h>

static DECLARE_SPINLOCK(lock);
static hashmap_t symbol_table;

static symbol_t *symbol_allocate(void)
{
    symbol_t *symbol = malloc(sizeof(symbol_t));
    if (symbol == NULL)
        return NULL;
    
    hashmap_node_init(&symbol->node);
    return symbol;
}

_init void symbol_init(struct mb_info *mb_info)
{
    const elf_shdr_t *symtab = mb_get_section(mb_info, ".symtab");
    const elf_shdr_t *strtab = mb_get_section(mb_info, ".strtab");
    if (symtab == NULL) 
        panic("No symbol table found");
    if (symtab == NULL)
        panic("No string table found");

    const unsigned int count = symtab->size / symtab->entsize;
    const elf_sym_t *symbols = (elf_sym_t *) symtab->addr;
    const char *names = (const char *) strtab->addr;

    hashmap_creat(&symbol_table, SYMBOLS_HASHMAP_LENGTH);
    for (size_t i = 0; i < count; i++) {
        const elf_sym_t *sym = &symbols[i];
        const char *name = (const char *) ((paddr_t) names + sym->name);
        // Only add global visible functions and variables
        if (ELF_ST_TYPE(sym->info) != ELF_STT_FUNC &&
            ELF_ST_TYPE(sym->info) != ELF_STT_OBJECT)
            continue;
        if (ELF_ST_BIND(sym->info) != ELF_STB_GLOBAL)
            continue;
        if (sym->other != ELF_STV_DEFAULT)
            continue;
        symbol_add(name, sym->value);
    }
}

/**
 * @brief Remove a symbol from the symbol table
 * 
 * @param name The name of the symbol to remove
 * @return int 0 if the symbol was removed or
 *  -ENOENT if the symbol does not exist
 */
int symbol_remove(const char *name)
{
    spin_acquire(&lock) {
        hashmap_foreach_result(&symbol_table, (unsigned int) name, entry) {
            symbol_t *symbol = container_of(entry, symbol_t, node);
            if (strcmp(symbol->name, name) == 0) {
                hashmap_remove(&symbol->node);
                free(symbol);
                return 0;
            }
        }
    }
    return -ENOENT;
}

/**
 * @brief Check if a symbol exists
 * 
 * @param name Name of the symbol to check
 * @return true if the symbol exists or
 *  false if the symbol does not exist
 */
bool symbol_exists(const char *name)
{
    return !!symbol_get_value(name);
}

/**
 * @brief Get the value of a symbol
 * 
 * @param name The name of the symbol
 * @return vaddr_t The value of the symbol or
 *  0 if the symbol does not exist.
 */
vaddr_t symbol_get_value(const char *name)
{
    hashmap_foreach_result(&symbol_table, strhash(name), entry) {
        symbol_t *symbol = container_of(entry, symbol_t, node);
        if (strcmp(symbol->name, name) == 0)
            return symbol->value;
    }
    return 0;
}

/**
 * @brief Add a symbol to the symbol table
 * 
 * @param name The name of the symbol. It must be a valid C identifier and
 * must not be already in the symbol table.
 * @param value The value of the symbol.
 * @return int 0 if the symbol was added or
 *  -EINVAL if the value of the symbol is 0 or
 *  -ENOMEM if malloc failed to allocate memory
 *  -EEXIST if the symbol already exist
 */
int symbol_add(const char *name, const vaddr_t value)
{
    if (symbol_exists(name))
        return -EEXIST;
    if (value == 0)
        return -EINVAL;

    symbol_t *symbol = symbol_allocate();
    if (symbol == NULL)
        return -ENOMEM;
    symbol->value = value;
    symbol->name = strdup(name);
    if (symbol->name == NULL) {
        free(symbol);
        return -ENOMEM;
    }

    spin_acquire(&lock) {
        hashmap_insert(&symbol_table, strhash(symbol->name), &symbol->node);
    }
    return 0;
}
