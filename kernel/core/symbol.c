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
#include <core/mm/malloc.h>

static DECLARE_SPINLOCK(lock);
static hashmap_t symbol_table;

_init void symbol_init(struct mb_info *mb_info)
{
    const elf_shdr_t *symtab = mb_get_section(mb_info, ".symtab");
    if (symtab == NULL) 
        panic("No symbol table found");
    const elf_shdr_t *strtab = mb_get_section(mb_info, ".strtab");
    if (symtab == NULL)
        panic("No string table found");

    const unsigned int count = symtab->size / symtab->entsize;
    const elf_sym_t *symbols = (elf_sym_t *) symtab->addr;
    const char *names = (const char *) strtab->addr;

    trace("Found %u symbols from kernel", count);
    hashmap_creat(&symbol_table, SYMBOLS_HASHMAP_LENGTH);
    for (size_t i = 0; i < count; i++) {
        const elf_sym_t *sym = &symbols[i];
        const char *name = (const char *) ((paddr_t) names + sym->name);

        // If the symbol is not a function or variable, skip it
        if (ELF_ST_TYPE(sym->info) != ELF_STT_FUNC &&
            ELF_ST_TYPE(sym->info) != ELF_STT_OBJECT)
            continue;
        // If the symbol is hidden, skip it
        if (ELF_ST_BIND(sym->info) == ELF_STB_LOCAL)
            continue;
        symbol_add(name, sym->value);
    }
}

/**
 * @brief Remove a symbol from the symbol table, currently unimplemented
 * 
 * @param name The name of the symbol to remove
 * @return int 0 if the symbol was removed
 * @return int -1 otherwise
 */
int symbol_remove(const char *name)
{
    unimplemented();
    symbol_t *symbol = symbol_get(name);
    if (symbol == NULL)
        return -1;
    hashmap_remove(&symbol->node);
    free(symbol);
    return 0;
}

/**
 * @brief Check if a symbol exists
 * 
 * @param name Name of the symbol to check
 * @return true if the symbol exists
 * @return false if the symbol does not exist
 */
bool symbol_exists(const char *name)
{
    return !!symbol_get(name);
}

/**
 * @brief Get the symbol associated to the given name.
 * 
 * @param name The name of the symbol to get.
 * @return struct symbol* The symbol associated to the given name, or NULL 
 * if it doesn't exist.
 */
struct symbol *symbol_get(const char *name)
{
    // Find the symbol in the hashmap
    spin_lock(&lock);
    hashmap_foreach_result(&symbol_table, (unsigned int) name, entry) {
        symbol_t *symbol = container_of(entry, symbol_t, node);
        if (strcmp(symbol->name, name) == 0) {
            spin_unlock(&lock);
            return symbol;
        }
    }
    spin_unlock(&lock);
    return NULL;
}

/**
 * @brief Add a symbol to the symbol table
 * 
 * @param name The name of the symbol. It must be a valid C identifier and
 * must not be already in the symbol table.
 * @param value The value of the symbol.
 * @return int 0 if the symbol was added, -1 otherwise (malloc error)
 */
int symbol_add(const char *name, const vaddr_t value)
{
    symbol_t *symbol = malloc(sizeof(symbol_t));
    if (symbol == NULL)
        return -1;
    symbol->name = strdup(name);
    symbol->value = value;
    hashmap_node_init(&symbol->node);

    spin_lock(&lock);
    hashmap_insert(&symbol_table, (unsigned int) symbol->name, &symbol->node);
    spin_unlock(&lock);
}
