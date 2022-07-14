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
// Disable some checks in kernel: assume kernel & modules are bug-free
#define CONFIG_DISABLE_CHECKS
#define CONFIG_EXTRA_CHECKS         // Enable extra checks to improve security
#define CONFIG_VSNPRINTF_64BITS     // Enable parsing 64 bits numbers
#define CONFIG_LOG                  // Enable logging (bochs only)
#define CONFIG_SMP                  // Enable SMP (unsupported now)
#define CONFIG_DEBUG_PANIC          // Enable panic with debug information
