
#include <stdlib.h>
#include <sys/types.h>
#include <stdbool.h>
#include <string.h>

typedef enum _microwave_state_t {
    MS_CLOSEDNOTIME = 0,
    MS_OPENTIME = 1,
    MS_OPENNOTIME = 2,
    MS_CLOSEDTIMENOMTRON = 3,
    MS_CLOSEDTIMEMTRON = 4,
} microwave_state;

struct microwave {
    microwave_state state;
    size_t time;
};

struct microwave *
new_microwave() {
    // This zeros the struct
    return calloc(1, sizeof(struct microwave));
}

void
reset_microwave(struct microwave *mwave) {
    memset(mwave, 0, sizeof(struct microwave));
}

bool
magnetron_enabled_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_CLOSEDTIMEMTRON:
            return true;
            break;
        default:
            break;
    }
    return false;
}

bool
door_open_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_OPENNOTIME:
            return true;
            break;
        case MS_OPENTIME:
            return true;
            break;
        default:
            break;
    }
    return false;
}

size_t
time_remain_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_OPENTIME:
            return mwave->time;
            break;
        case MS_CLOSEDTIMENOMTRON:
            return mwave->time;
            break;
        case MS_CLOSEDTIMEMTRON:
            return mwave->time;
            break;
        default:
            break;
    }
    return 0;
}

void
tick_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_CLOSEDTIMEMTRON:
            if (mwave->time > 0) {
                mwave->time -= 1;
            }
            if (mwave->time == 0) {
                mwave->state = MS_CLOSEDTIMENOMTRON;
            }
            break;
        default:
            break;
    }
}


void
action_open_door_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_CLOSEDNOTIME:
            mwave->state = MS_OPENNOTIME;
            break;
        case MS_CLOSEDTIMENOMTRON:
            mwave->state = MS_OPENTIME;
            break;
        case MS_CLOSEDTIMEMTRON:
            mwave->state = MS_OPENTIME;
            break;
        default:
            break;
    }
}

void
action_close_door_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_OPENNOTIME:
            mwave->state = MS_CLOSEDNOTIME;
            break;
        case MS_OPENTIME:
            mwave->state = MS_CLOSEDTIMENOMTRON;
            break;
        default:
            break;
    }
}

void
action_set_time_microwave(struct microwave *mwave, size_t time) {
    switch(mwave->state) {
        case MS_OPENNOTIME:
            mwave->state = MS_OPENTIME;
            mwave->time = time;
            break;
        case MS_OPENTIME:
            mwave->time = time;
            break;
        case MS_CLOSEDNOTIME:
            mwave->state = MS_CLOSEDTIMENOMTRON;
            mwave->time = time;
            break;
        case MS_CLOSEDTIMENOMTRON:
            mwave->time = time;
            break;
        default:
            break;
    }
}

void
action_start_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_CLOSEDNOTIME:
            mwave->state = MS_CLOSEDTIMEMTRON;
            mwave->time = 30;
            break;
        case MS_CLOSEDTIMENOMTRON:
            mwave->state = MS_CLOSEDTIMEMTRON;
            break;
        case MS_CLOSEDTIMEMTRON:
            mwave->time += 30;
            break;
        default:
            break;
    }
}

void
action_stop_microwave(struct microwave *mwave) {
    switch(mwave->state) {
        case MS_OPENTIME:
            mwave->state = MS_OPENNOTIME;
            break;
        case MS_CLOSEDTIMENOMTRON:
            mwave->state = MS_CLOSEDNOTIME;
            break;
        case MS_CLOSEDTIMEMTRON:
            mwave->state = MS_CLOSEDTIMENOMTRON;
            break;
        default:
            break;
    }
}



