#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PART_ONE_LENGTH 4
#define PART_TWO_LENGTH 14
#define ALPHABET 26

char *small_txt = "../data/small.txt";
char *large_txt = "../data/full.txt";

int main(int argc, char *argv[]) {
    FILE *fp;
    char *filename;

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

    char *buffer = 0;
    long length;
    fseek(fp, 0, SEEK_END);
    length = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    buffer = malloc(length);
    if (buffer) {
        fread(buffer, 1, length, fp);
    }
    fclose(fp);

    int i;
    for (i = PART_ONE_LENGTH - 1; i < length; i++) {
        char contents[ALPHABET];
        for (int j = 0; j < ALPHABET; j++) {
            contents[j] = 0;
        }

        for (int j = i; j >= i - (PART_ONE_LENGTH - 1); j--) {
            contents[buffer[j] - 'a'] = 1;
        }

        int sum = 0;
        for (int j = 0; j < ALPHABET; j++) {
            sum += contents[j];
        }

        if (sum == PART_ONE_LENGTH) {
            break;
        }
    }

    printf("Part One: %d\n", i + 1);

    for (i = PART_TWO_LENGTH - 1; i < length; i++) {
        char contents[ALPHABET];
        for (int j = 0; j < ALPHABET; j++) {
            contents[j] = 0;
        }

        for (int j = i; j >= i - (PART_TWO_LENGTH - 1); j--) {
            contents[buffer[j] - 'a'] = 1;
        }

        int sum = 0;
        for (int j = 0; j < ALPHABET; j++) {
            sum += contents[j];
        }

        if (sum == PART_TWO_LENGTH) {
            break;
        }
    }

    printf("Part Two: %d\n", i + 1);
}
