#include <stdio.h>
#include <unistd.h>

void vulnerable_function();
void win();

void win() { puts("You win!"); }

int main() {
  puts("About to call vulnerable function");

  vulnerable_function();

  puts("About to return from main");
}

struct my_struct {
  char username[64];
  int always_false;
  char other_data[128];
};

void vulnerable_function() {
  struct my_struct s;
  s.always_false = 0;

  puts("what is your username?");
  read(0, s.username, 300);

  printf("Hello %s\n", s.username);

  if (s.always_false) {
    puts("Always false is true!");
  } else {
    puts("Always false is false!");
  }
}
