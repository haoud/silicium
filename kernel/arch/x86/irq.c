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
#include <arch/x86/idt.h>
#include <arch/x86/irq.h>

#define install_irq(i) ({                       \
    extern void irq_##i(void);                  \
    set_interrupt_gate(IRQ_BASE + i, &irq_##i); \
})

static irq_handler_t irq_handlers[IRQ_MAX];

_init
void irq_install(void)
{
    install_irq(0);
    install_irq(1);
    install_irq(2);
    install_irq(3);
    install_irq(4);
    install_irq(5);
    install_irq(6);
    install_irq(7);
    install_irq(8);
    install_irq(9);
    install_irq(10);
    install_irq(11);
    install_irq(12);
    install_irq(13);
    install_irq(14);
    install_irq(15);

    for (unsigned int i = 0; i < IRQ_MAX; i++)
        irq_handlers[i] = NULL;
}

/**
 * @brief Request an IRQ handler for an IRQ.
 * For now, only one handler can be installed for an IRQ. It may be changed
 * in the future.
 * 
 * @param irq The IRQ number to request
 * @param handler The handler to call when the IRQ is raised
 * @param name The name of the handler (for debugging, unused)
 * @param flags Flags for the function (unused)
 * @return int 0 on success or
 *  -EBUSY if the IRQ is already used
 */
_export int irq_request(
	const unsigned int irq,
	const irq_handler_t handler,
	const char *name,
	const int flags)
{
    assert(irq < IRQ_MAX);
    if (irq_handlers[irq] != NULL)
        return -EBUSY;

    irq_handlers[irq] = handler;
    return 0;
}

/**
 * @brief The handler for the IRQs. It calls the handler for the IRQ
 * if it exists. For now, it is very simple and it can only handle one
 * handler per IRQ.
 * 
 * @param state The CPU state
 */
_asmlinkage
void irq_handler(cpu_state_t *state)
{
    assert(state != NULL);
    assert(state->data < IRQ_MAX);

    if (irq_handlers[state->data] != NULL)
        irq_handlers[state->data](state);
}
