#include <stdio.h>
#include <stdlib.h>

int *merge(int *left_array, int left_size, int *right_array, int right_size)
{
    int new_size = left_size + right_size;
    int *new_array = malloc(sizeof(int) * new_size);
    int i = 0;
    int j = 0;
    int k = 0;

    for (; (i < left_size) && (j < right_size) && (k < new_size); k++)
    {
        int left_element = left_array[i];
        int right_element = right_array[j];

        if (left_element >= right_element)
        {
            new_array[k] = left_element;
            i++;
        }
        else
        {
            new_array[k] = right_element;
            j++;
        }
    }

    if (k < new_size)
    {
        if (i < left_size)
        {
            for (; k < new_size; k++)
            {
                new_array[k] = left_array[i];
                i++;
            }
        }
        else
        {
            for (; k < new_size; k++)
            {
                new_array[k] = right_array[j];
                j++;
            }
        }
    }

    return new_array;
}

int *sort(int *array, int size)
{
    if (size < 2)
        return array;

    int new_size = size / 2;

    return merge(
        sort(array, new_size),
        new_size,
        sort(array + new_size, size - new_size),
        size - new_size);
}

int main()
{
    int size = 15000;
    int unsorted[size];
    printf("Unsorted: [");
    for (int i = 0; i < size; i++)
    {
        int new = rand() % 1000;
        printf("%d, ", new);
        unsorted[i] = new;
    }
    printf("]\n");

    int *sorted = sort(unsorted, size);
    printf("Sorted: [");
    for (int i = 0; i < size; i++)
    {
        printf("%d, ", sorted[i]);
    }
    printf("]\n");
}