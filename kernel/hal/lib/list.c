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
#include <lib/list.h>

void list_insert(struct list_head *const prev,
				 struct list_head *const next,
				 struct list_head *const entry)
{
	next->prev = entry;
	entry->next = next;
	entry->prev = prev;
	prev->next = entry;
}

int list_empty(struct list_head *const list)
{
	return (list->next == list);
}

void list_init(struct list_head *const list)
{
	list->prev = list->next = list;
}

void list_entry_init(struct list_head *const list)
{
	list->prev = list->next = list;
}

void list_remove(struct list_head *const entry)
{
	entry->prev->next = entry->next;
	entry->next->prev = entry->prev;
	entry->prev = entry;
	entry->next = entry;
}

void list_add(struct list_head *const list, struct list_head *const entry)
{
	list_add_tail(list, entry);
}

void list_add_head(struct list_head *const list, struct list_head *const entry)
{
	list_insert(list, list->next, entry);
}

void list_add_tail(struct list_head *const list, struct list_head *const entry)
{
	list_insert(list->prev, list, entry);
}
