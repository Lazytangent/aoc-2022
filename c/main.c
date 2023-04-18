#include <stdio.h>
#include <string.h>

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
}
