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

typedef void (*timer_callback_t)(void *);

typedef struct timer {
    timer_callback_t callback;
    time_t expire;
    bool active;
    void *data;
    struct list_head node;
} timer_t;

void timer_tick(void);
void timer_init(timer_t *timer);

int timer_add(timer_t *timer);
int timer_remove(timer_t *timer);
bool timer_expired(timer_t *timer);
int timer_expire(timer_t *timer, time_t expire);
int timer_update(timer_t *timer, time_t expire);