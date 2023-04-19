#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

using namespace std;

const string SMALL_TXT = "../data/small.txt";
const string LARGE_TXT = "../data/full.txt";

int get_value(char);

int main(int argc, char* argv[]) {
    string filename = SMALL_TXT;
    if (argc > 1) {
        string arg = argv[1];
        transform(arg.begin(), arg.end(), arg.begin(), ::tolower);
        if (arg == "full") {
            filename = LARGE_TXT;
        }
    }

    ifstream t(filename);
    stringstream buffer;
    buffer << t.rdbuf();

    string line;

    int sum = 0;
    while (getline(buffer, line, '\n')) {
        int n = line.length();
        int half_index = n / 2;
        string first_half = line.substr(0, half_index);
        string second_half = line.substr(half_index, half_index);

        for (int i = 0; i < first_half.length(); ++i) {
            if (second_half.find(first_half[i]) != string::npos) {
                sum += get_value(first_half[i]);
                break;
            }
        }
    }

    cout << "Part One: " << sum << endl;
}

int get_value(char c) {
    if (c >= 'a' && c <= 'z') {
        return c - 'a' + 1;
    }
    return c - 'A' + 27;
}
