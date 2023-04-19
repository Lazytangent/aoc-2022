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
int calculate_part_two(char opponent, char me);
int lose_against(char opponent);
int tie_against(char opponent);
int win_against(char opponent);

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
    int part_two = 0;
    while (fgets(line, 5, fp) != NULL) {
        line[strcspn(line, "\n")] = 0;

        char opponent = line[0];
        char me = line[2];

        score += calculate_round_one(opponent, me);
        part_two += calculate_part_two(opponent, me);
    }

    printf("Part One: %d\n", score);
    printf("Part Two: %d\n", part_two);
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

int calculate_part_two(char opponent, char me) {
    int score = 0;

    switch (me) {
        case 'X':
            score += lose_against(opponent);
            break;
        case 'Y':
            score += tie_against(opponent);
            break;
        case 'Z':
            score += win_against(opponent);
            break;
    }

    return score;
}

int lose_against(char opponent) {
    switch (opponent) {
        case 'A':
            return LOSS + SCISSORS;
        case 'B':
            return LOSS + ROCK;
        case 'C':
            return LOSS + PAPER;
    }
}

int tie_against(char opponent) {
    switch (opponent) {
        case 'A':
            return TIE + ROCK;
        case 'B':
            return TIE + PAPER;
        case 'C':
            return TIE + SCISSORS;
    }
}

int win_against(char opponent) {
    switch (opponent) {
        case 'A':
            return WIN + PAPER;
        case 'B':
            return WIN + SCISSORS;
        case 'C':
            return WIN + ROCK;
    }
}
