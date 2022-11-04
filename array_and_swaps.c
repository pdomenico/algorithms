#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

struct test
{
    int n;
    int k;
    int *a;
    int *b;
};

int *sort(int *array, int size)
{
    for (int i = 0; i < size; i++)
    {
        bool swapped = false;
        for (int j = 0; j < size - i - 1; j++)
        {
            if (array[j] < array[j + 1])
            {
                int temp = array[j + 1];
                array[j + 1] = array[j];
                array[j] = temp;
                swapped = true;
            }
        }
        if (!swapped)
            break;
    }
    return array;
}

int sum_array(int *array, int size)
{
    int sum = 0;
    for (int i = 0; i < size; i++)
    {
        sum += array[i];
    }
    return sum;
}

int main()
{
    char test_n_str[5];
    fgets(test_n_str, 5, stdin);
    int test_n = atoi(test_n_str);

    struct test current_test;
    current_test.a = malloc(30 * sizeof(int));
    current_test.b = malloc(30 * sizeof(int));
    int *solutions = malloc(test_n * sizeof(int));

    for (int i = 0; i < test_n; i++)
    {
        // read n and k
        char *ptr;
        char line[15];
        fgets(line, 15, stdin);
        current_test.n = strtol(line, &ptr, 10);
        current_test.k = strtol(ptr, &ptr, 10);

        // read a and b
        char a_elements[current_test.n * 3];
        char b_elements[current_test.n * 3];
        fgets(a_elements, sizeof(a_elements), stdin);
        fgets(b_elements, sizeof(b_elements), stdin);

        char *a_ptr = a_elements;
        char *b_ptr = b_elements;
        for (int j = 0; j < current_test.n; j++)
        {
            current_test.a[j] = strtol(a_ptr, &a_ptr, 10);
            current_test.b[j] = strtol(b_ptr, &b_ptr, 10);
        }
        // printf("a_elements: %s", a_elements);
        // printf("unsorted array A: [");
        // for (int j = 0; j < current_test.n; j++)
        //     printf("%d, ", current_test.a[j]);
        // printf("]\nb_elements: %s", b_elements);
        // printf("\nunsorted array B: [");
        // for (int j = 0; j < current_test.n; j++)
        //     printf("%d, ", current_test.b[j]);
        // printf("]\n");

        // sort a and b
        current_test.a = sort(current_test.a, current_test.n);
        current_test.b = sort(current_test.b, current_test.n);

        // printf("sorted array A: [");
        // for (int j = 0; j < current_test.n; j++)
        //     printf("%d, ", current_test.a[j]);
        // printf("]\nsorted array B: [");
        // for (int j = 0; j < current_test.n; j++)
        //     printf("%d, ", current_test.b[j]);
        // printf("]\n");

        // compute the solution
        int n_of_swaps = 0;
        solutions[i] = 0;
        for (int j = current_test.n - 1; j >= 0; j--)
        {
            if (n_of_swaps < current_test.k)
            {
                if (current_test.a[j] < current_test.b[n_of_swaps])
                {
                    solutions[i] += current_test.b[n_of_swaps];
                    // printf("Swappping %d with %d\n", current_test.a[j], current_test.b[n_of_swaps]);
                    n_of_swaps++;
                }
                else
                {
                    solutions[i] += current_test.a[j];
                    // printf("Preferring %d to %d\n", current_test.a[j], current_test.b[n_of_swaps]);
                }
            }
            else
            {
                solutions[i] += current_test.a[j];
                // printf("Summing %d\n", current_test.a[j]);
            }
        }
    }

    for (int i = 0; i < test_n; i++)
        printf("%d\n", solutions[i]);

    free(current_test.a);
    free(current_test.b);
    free(solutions);
}