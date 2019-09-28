
#include <stdlib.h>
#include <sys/types.h>
#include <stdbool.h>
#include <string.h>

struct microwave {
};

struct microwave *
new_microwave() {
    return NULL;
}

void
reset_microwave(struct microwave *mwave) {
}

bool
magnetron_enabled_microwave(struct microwave *mwave) {
    return false;
}

bool
door_open_microwave(struct microwave *mwave) {
    return false;
}

size_t
time_remain_microwave(struct microwave *mwave) {
    return 0;
}

void
tick_microwave(struct microwave *mwave) {
}


void
action_open_door_microwave(struct microwave *mwave) {
}

void
action_close_door_microwave(struct microwave *mwave) {
}

void
action_set_time_microwave(struct microwave *mwave, size_t time) {
}

void
action_start_microwave(struct microwave *mwave) {
}

void
action_stop_microwave(struct microwave *mwave) {
}



