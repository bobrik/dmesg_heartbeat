#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/module.h>

MODULE_DESCRIPTION("Print heartbeat into dmesg on a timer");
MODULE_AUTHOR("Ivan Babrou <dmesg_heartbeat@ivan.computer>");
MODULE_LICENSE("GPL");

#define HEARTBEAT_INTERVAL HZ * 10

struct timer_list heartbeat_timer;

static void arm_timer(struct timer_list *timer)
{
    mod_timer(&heartbeat_timer, jiffies + HEARTBEAT_INTERVAL);
}

static void heartbeat(struct timer_list *timer)
{
    pr_info("🫀\n");
    arm_timer(timer);
}

static int dmesg_heartbeat_init(void)
{
    timer_setup(&heartbeat_timer, heartbeat, 0);
    arm_timer(&heartbeat_timer);
    return 0;
}

static void dmesg_heartbeat_exit(void)
{
    timer_delete_sync(&heartbeat_timer);
}

module_init(dmesg_heartbeat_init);
module_exit(dmesg_heartbeat_exit);
