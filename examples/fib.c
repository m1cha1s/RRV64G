int fib(int n);

int test();

int main() {
    return fib(1000000);
	//return test();
	//return 123456;
}

int test() {
	return 139586;
}
int fib(int n) {
    if (n == 0 || n == 1)
        return n;
    else
        return (fib(n-1) + fib(n-2));
}
