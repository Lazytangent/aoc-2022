#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *small_txt = "../data/small.txt";
char *large_txt = "../data/full.txt";

int main(int argc, char *argv[]) {
    FILE *fp;
    char *filename;
    char line[100];

    if (argc > 1 && strcmp(argv[1], "full") == 0) {
        filename = large_txt;
    } else {
        filename = small_txt;
    }

    fp = fopen(filename, "r");

    if (fp == NULL) {
        printf("Error! Could not read data file: %s\n", filename);
        exit(-1);
    }

    int max = 0;
    int current = 0;
    int second = 0, third = 0;
    while (fgets(line, 100, fp) != NULL) {
        line[strcspn(line, "\n")] = 0;
        if (strlen(line) == 0) {
            if (current > max) {
                third = second;
                second = max;
                max = current;
            } else if (current > second) {
                third = second;
                second = current;
            } else if (current > third) {
                third = current;
            }

            current = 0;
            continue;
        }

        int amount = atoi(line);
        current += amount;
    }

    if (current > max) {
        third = second;
        second = max;
        max = current;
    } else if (current > second) {
        third = second;
        second = current;
    } else if (current > third) {
        third = current;
    }

    printf("Part One: %d\n", max);
    printf("Part Two: %d\n", max + second + third);
}
