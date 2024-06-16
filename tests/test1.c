int main() {


//array fib stores numbers of fibonacci series
  int i, fib[10];
// initialized first element to 0
  fib[0] = 0;
// initialized second element to 1
  fib[1] = 1;
//loop to generate ten elements
  for (i = 2; i < 10; i++)
    {
//i'th element of series is equal to the sum of i-1'th element and i-2'th element.
      fib[i] = fib[i - 1] + fib[i - 2];
    }

//print all numbers in the series
  

  return 0;



}
