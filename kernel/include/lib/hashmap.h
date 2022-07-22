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
#pragma once
#include <kernel.h>
#include <lib/list.h>

#define hashmap_foreach_result(map, key, name) \
    list_foreach(hashmap_get(map, key), name)

#define hashmap_foreach(map, name)                         \
    for (unsigned int __i = 0; __i < (map)->length; __i++) \
        hashmap_foreach_result(map, __i, name)

typedef struct hash_node {
    struct list_head node;
} hash_head_t;

typedef struct hashmap {
    unsigned int length;
    struct hash_node *entries;
} hashmap_t;

void hashmap_destroy(hashmap_t *map);
void hashmap_remove(struct hash_node *head);
int hashmap_creat(hashmap_t *map, const unsigned int length);
void hashmap_insert(hashmap_t *map, unsigned int key, struct hash_node *head);

struct list_head *hashmap_get(hashmap_t *map, unsigned int key);
