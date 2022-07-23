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
#include <lib/hashmap.h>
#include <core/mm/malloc.h>

/**
 * @brief Destroy a hashmap and free the memory allocated for it.
 * The objects inserted in the hash table are not destroyed: it is up
 * to the user of this function to destroy them if necessary
 * 
 * @param map The hashmap to destroy.
 */
void hashmap_destroy(hashmap_t *map)
{
    free(map->entries);
    map->entries = NULL;
    map->length = 0;
}

void hashmap_node_init(hash_node_t *node)
{
    list_entry_init(&node->node);
}

/**
 * @brief Create a hashmap with a given length.
 * 
 * @param map The hashmap to initialize.
 * @param length The length of the hashmap.
 * @return int 0 on success, -1 on error
 */
int hashmap_creat(hashmap_t *map, const unsigned int length)
{
    map->entries = malloc(sizeof(struct hash_node) * length);
    if (map->entries == NULL)
        return -1;
    for (unsigned int i = 0; i < length; i++)
        list_init(&map->entries[i].node);

    map->length = length;
    return 0;
}

/**
 * @brief Remove a node from a hashmap
 * 
 * @param head The node to remove
 */
void hashmap_remove(struct hash_node *head)
{
    list_remove(&head->node);
}

/**
 * @brief Insert a node into the hashmap with the given key. The key must
 * be unique and must not already be inserted in the hash table (the behavior
 * of the table is undefined in this case).
 * The node is inserted at the end of the list if a collision occurs.
 * 
 * @param map The hashmap
 * @param key The key of the node: must be unique
 * @param head The node to insert
 */
void hashmap_insert(hashmap_t *map, unsigned int key, struct hash_node *head)
{
    const unsigned int index = key % map->length;
    list_add_tail(&map->entries[index].node, &head->node);
}

/**
 * @brief Returns the list of hash_node associated to the key. Two objects 
 * with two different keys can be assigned to the same location in the hash
 * table (collision).
 * This function cannot know which object is really associated with the key:
 * it is up to the user to find the right object among the returned list
 * 
 * @param map Hashmap to search in.
 * @param key Key to search for.
 * @return struct list_head* A list of hash_node.
 */
struct list_head *hashmap_get(hashmap_t *map, unsigned int key)
{
    return &map->entries[key % map->length].node;
}