/*
 * A simple rtems program to show how to use configuration templates
 */

#include <rtems.h>
/* configuration information */

#define CONFIGURE_INIT

#include <bsp.h> /* for device driver prototypes */

rtems_task Init( rtems_task_argument argument);	/* forward declaration needed */

/* configuration information */

#define CONFIGURE_APPLICATION_NEEDS_CONSOLE_DRIVER
#define CONFIGURE_APPLICATION_NEEDS_CLOCK_DRIVER

#define CONFIGURE_MAXIMUM_TASKS             4

#define CONFIGURE_RTEMS_INIT_TASKS_TABLE
#define CONFIGURE_MAXIMUM_FILE_DESCRIPTORS 32

#define CONFIGURE_EXTRA_TASK_STACKS         (3 * RTEMS_MINIMUM_STACK_SIZE)


#include <rtems/confdefs.h>

/* If --drvmgr was enabled during the configuration of the RTEMS kernel */
#ifdef RTEMS_DRVMGR_STARTUP
 #ifdef LEON3
  /* Add Timer and UART Driver for this example */
  #ifdef CONFIGURE_APPLICATION_NEEDS_CLOCK_DRIVER
   #define CONFIGURE_DRIVER_AMBAPP_GAISLER_GPTIMER
  #endif
  #ifdef CONFIGURE_APPLICATION_NEEDS_CONSOLE_DRIVER
   #define CONFIGURE_DRIVER_AMBAPP_GAISLER_APBUART
  #endif
 #endif

 #include <drvmgr/drvmgr_confdefs.h>
#endif

#include <rtems/bspIo.h>
#include <stdio.h>
#include <stdlib.h>

extern int rust_entry(void);

rtems_task Init(
  rtems_task_argument ignored
)
{
  iprintf( "Hello World\n" );
  rtems_task_wake_after(100);
  rust_entry();
  printk( "Hello World over printk() on Debug console\n\n" );
  exit( 0 );
}
