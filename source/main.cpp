#include "gpio_api.h"
#include "wait_api.h"

const int PERIOD_MS = 500;
const int ON_MS = 30;

gpio_t row_1;
gpio_t col_1;

int main()
{
    gpio_init_out_ex(&row_1, ROW1, 1);
    gpio_init_out(&col_1, COL1);
    for (;;) {
        gpio_write(&col_1, 0);
        wait_ms(ON_MS);
        gpio_write(&col_1, 1);
        wait_ms(PERIOD_MS - ON_MS);
    }
}

