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
#include <process/queue.h>

/**
 * @brief Initialize a thread queue.
 * 
 * @param queue The thread queue to initialize.
 */
void thread_queue_init(thread_queue_t *queue)
{
    assume(!null(queue));
    list_init(&queue->node);
    spin_init(&queue->lock);
}

/**
 * @brief Insert a thread queue entry in a thread queue.
 * 
 * @param queue The thread queue to insert the entry in.
 * @param entry The thread to insert.
 */
void thread_queue_insert(thread_queue_t *queue, thread_queue_t *entry)
{
    assume(!null(queue));
    assume(!null(entry));
    assume(list_empty(&entry->node));

    spin_acquire(&queue->lock) {
        list_add(&queue->node, &entry->node);
    }
}

/**
 * @brief Remove a thread from a thread queue.
 * 
 * @param queue The thread queue to remove the entry from.
 * @param entry The entry to remove.
 */
void thread_queue_remove(thread_queue_t *queue, thread_queue_t *entry)
{
    assume(!null(queue));
    assume(!null(entry));
    assume(!list_empty(&entry->node));

    spin_acquire(&queue->lock) {
        list_remove(&entry->node);
    }
}
