void print(char* st);
void println(char* st);

int main() {
  println("Hello world!");
  return 0;
}

void print(char* st) {
  volatile char *uart = (volatile char *) 0x10000000;
  for(int i = 0; i < (sizeof(st)/sizeof(char)); i ++) {
    uart[0] = st[i];
  }  
}

void println(char* st) {
  volatile char *uart = (volatile char *) 0x10000000;
  print(st);
  uart[0] = '\n';
}
