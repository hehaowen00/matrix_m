#include <stdlib.h>
#include <stdint.h>

void matrix_mult(
    float *dest,
    float *a1, size_t a_cols, size_t a_rows,
    float *b1, size_t b_cols, size_t b_rows)
{
    for (int i = 0; i < a_rows; i++)
    {
        for (int j = 0; j < b_cols; j++)
        {
            float temp = 0;
            for (int k = 0; k < b_rows; k++)
            {
                float a = a1[i * a_cols + k];
                float b = b1[k * b_cols + j];
                temp = temp + (a * b);
            }
            dest[i * b_cols + j] = temp;
        }
    }
}
