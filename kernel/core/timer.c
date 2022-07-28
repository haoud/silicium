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
#include <core/timer.h>
#include <lib/spinlock.h>
#include <arch/x86/time.h>

static DECLARE_SPINLOCK(lock);
static DECLARE_LIST(timers);

/**
 * @brief This function is called every hardware tick to check if any timer
 * has expired.
 * 
 * This function has some problems: At each call, it will check all the
 * list of timers to check if the timer is expired: The performance of
 * this function could be improved if the list was sorted by expiration
 * time.
 * The second problem is that we look the timer list, potentially for
 * a long time because we doen't now how long the callbacks will take to run.
 * It should be improved too.
 */
void timer_tick(void)
{
    spin_acquire(&lock) {
        list_foreach(&timers, entry) {
            timer_t *timer = container_of(entry, timer_t, node);
            if (timer_expired(timer)) {
                timer_remove(timer);
                timer->callback(timer->data);
            }
        }
    }
}

/**
 * @brief Initialise a timer. It set the timer as inactive and initialize the
 * list node. Other field are untouched and must be set by the caller in order
 * to use the timer after this function.
 * 
 * @param timer The timer to initialize.
 */
void timer_init(timer_t *timer)
{
    assume(!null(timer));
    list_init(&timer->node);
    timer->active = false;
}

/**
 * @brief Add a timer to the list of active timers.
 * 
 * @param timer The timer to add.
 * @return int 0 if the timer was added, or
 *  -EEXIST if the timer is already active or
 *  -EAGAIN if the timer was expired. In this case, this function will run
 * the callback and return this error.
 */
int timer_add(timer_t *timer)
{
    assume(!null(timer));
    if (list_empty(&timer->node))
        return -EEXIST;
    if (timer_expired(timer)) {
        timer->callback(timer->data);
        return -EAGAIN;
    }

    timer->active = 1;
    spin_acquire(&lock) {
        list_add(&timers, &timer->node);
    }
    return 0;
}

/**
 * @brief Remove a timer from the list of active timers.
 * 
 * @param timer The timer to remove.
 * @return int 0 if the timer was removed or
 *  -ENOENT if the timer was not is the active timer list.
 */
int timer_remove(timer_t *timer)
{
    assume(!null(timer));
    if (list_empty(&timer->node))
        return -ENOENT;

    spin_acquire(&lock) {
        list_add(&timers, &timer->node);
    }
    timer->active = 0;
    return 0;
}

/**
 * @brief Check if a timer is expired. The timer must be active to call
 * this function, otherwise the behavior is undefined.
 * 
 * @param timer The timer to check.
 * @return true if the timer is expired.
 * @return false if the timer is not expired.
 */
bool timer_expired(timer_t *timer)
{
    assume(!null(timer));
    assume(timer->active);
    if (timer->expire <= time_startup_ms())
        return true;
    return false;
}

/**
 * @brief Set the expire time of a timer.
 * 
 * @param timer The timer to set.
 * @param expire The expiration time of the timer, in milliseconds since this
 * function is called. For example, if you want to set a timer to expire in
 * 1.5 second, you should call this function with the value 1500. 
 * @return int always 0.
 */
int timer_expire(timer_t *timer, time_t expire)
{
    assume(!null(timer));
    timer->expire = time_startup_ms() + expire;
    return 0;
}

/**
 * @brief Update the expire time of a timer. The expire time is set and the
 * timer is added to the list of active timers. If the timer is already active,
 * it will be removed from the list, updated and added again.
 * 
 * @param timer The timer to update.
 * @param expire The expiration time of the timer, in milliseconds (see 
 * timer_expire() for more informations).
 * @return int same error code as timer_add().
 */
int timer_update(timer_t *timer, time_t expire)
{
    timer_remove(timer);
    timer_expire(timer, expire);
    return timer_add(timer);
}
