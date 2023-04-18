#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *small_txt = "../data/small.txt";
char *large_txt = "../data/full.txt";

void debug(int argc, char *argv[]);

int main(int argc, char *argv[]) {
    FILE *fp;
    char *filename;
    char line[100];

    // debug(argc, argv);

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
    while (fgets(line, 100, fp) != NULL) {
        line[strcspn(line, "\n")] = 0;
        if (strlen(line) == 0) {
            if (current > max) {
                max = current;
            }

            current = 0;
            continue;
        }

        int amount = atoi(line);
        current += amount;
    }

    printf("Part One: %d\n", max);
}

void debug(int argc, char *argv[]) {
    printf("argc = %d\n", argc);

    for (int i = 0; i < argc; i++) {
        printf("argv[%d] = %s\n", i, argv[i]);
    }

    if (argc > 1) {
        printf("argv[1] == \"full\" = %d\n", argv[1] == "full");
    }
}
