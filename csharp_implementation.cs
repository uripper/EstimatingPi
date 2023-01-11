using System;
using System.Diagnostics;

//Creating a C# implementation that estimates pi using the euclid algorithm, counting co-primes, finding the ratio of co-primes to total numbers, multiplying by 6, and getting the square root of that to get an estimate of pi.

namespace csharp_implementation
{
    class Program
    {
        static void Main(string[] args)
        {
            Stopwatch sw = new Stopwatch();
            sw.Start();
            int num = 9999999999;
            int count_co_primes = 1;
            int count_total = 1;
            Random rand = new Random();
            for (int i = 0; i < num; i++)
            {
                long a = rand.Next(1, num);
                long b = rand.Next(1, num);
                if (a < b)
                {
                    long temp = a;
                    a = b;
                    b = temp;
                }

                if (gcd(a, b) == 1)
                {
                    count_co_primes++;
                }
                count_total++;
            }
            double coprime = count_co_primes;
            double total = count_total;
            Console.WriteLine("count_co_primes: " + coprime + " count_total: " + total);
            Console.WriteLine("Ratio: " + (total / coprime));
            Console.WriteLine("Ratio * 6: " + (6 * (total / coprime)));
            double pi = Math.Sqrt(6 * (total / coprime));
            sw.Stop();
            Console.WriteLine("Estimated pi: " + pi);
            Console.WriteLine("Time elapsed: " + sw.ElapsedMilliseconds + " milliseconds");
        }

        static long gcd(long a, long b)
        {
            if (b == 0)
            {
                return a;
            }
            else
            {   
                return gcd(b, a % b);
            }
            
        }
    }
}
