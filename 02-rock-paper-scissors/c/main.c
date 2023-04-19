#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WIN 6
#define TIE 3
#define LOSS 0

#define ROCK 1
#define PAPER 2
#define SCISSORS 3

char *small_txt = "../data/small.txt";
char *large_txt = "../data/full.txt";

int calculate_round_one(char opponent, char me);
int against_rock(char me);
int against_paper(char me);
int against_scissors(char me);
int get_point_value(char me);

int main(int argc, char *argv[]) {
    FILE *fp;
    char *filename;
    char line[5];

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
    int score = 0;
    while (fgets(line, 5, fp) != NULL) {
        line[strcspn(line, "\n")] = 0;

        char opponent = line[0];
        char me = line[2];

        score += calculate_round_one(opponent, me);
    }

    printf("Part One: %d\n", score);
}

int calculate_round_one(char opponent, char me) {
    int score = 0;

    switch (opponent) {
        case 'A':
            score += against_rock(me);
            break;
        case 'B':
            score += against_paper(me);
            break;
        case 'C':
            score += against_scissors(me);
            break;
    }
    score += get_point_value(me);

    return score;
}

int against_rock(char me) {
    switch (me) {
        case 'X':
            return TIE;
        case 'Y':
            return WIN;
        case 'Z':
            return LOSS;
    }
}

int against_paper(char me) {
    switch (me) {
        case 'X':
            return LOSS;
        case 'Y':
            return TIE;
        case 'Z':
            return WIN;
    }
}

int against_scissors(char me) {
    switch (me) {
        case 'X':
            return WIN;
        case 'Y':
            return LOSS;
        case 'Z':
            return TIE;
    }
}

int get_point_value(char me) {
    switch (me) {
        case 'X':
            return ROCK;
        case 'Y':
            return PAPER;
        case 'Z':
            return SCISSORS;
    }
}
