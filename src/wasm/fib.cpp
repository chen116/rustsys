// int fib ( int n){
//     int i, t1 = 0, t2 = 1, nextTerm;

//     for (i = 1; i <= n; ++i) {
//         nextTerm = t1 + t2;
//         t1 = t2;
//         t2 = nextTerm;
//     }
// return t1;
// }

int fib(int n)
{
    if (n <= 1)
        return n;
    else
        return fib(n - 1) + fib(n - 2);
}