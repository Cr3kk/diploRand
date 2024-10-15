using System;
using System.Collections.Generic;

namespace diploRandCSharp
{
    class Program
    {
        Random random = new Random();
        public static int[] randomCall(int[] random_array, int length)
        {
            ran = random.Next(1, length);
            if (!random_array.Contains(ran))
            {
                random_array.Add(ran);
            }
            return random_array;
        }

        public static int[] createRandomArray(int length)
        {   
            int[] random_array = new int[length];
            while random_array.Length < length
            {
                random_array = randomCall(random_array, length);
            }
            return random_array;
        }   
        public static void Main()
        {
            Dictionary<string, string> names_previous_round = new Dictionary<string, string>()
            {
                {"Ben", "Turkey"},
                {"Casper", "Germany"},
                {"Niels", "Austria"},
                {"Koen", "Italy"},
                {"Lock", "England"},
                {"Jan-Jan", "Russia"},
                {"Wouter", "France"}
            };

            Dictionary<string, int> country_to_number = new Dictionary<string, int>()
            {
                {"France", 1},
                {"Russia", 2},
                {"Italy", 3},
                {"Germany", 4},
                {"England", 5},
                {"Austria", 6},
                {"Turkey", 7}
            };

            HashSet<int> previous_round_set = new HashSet<int>();
            for (int i = 0; i < names_previous_round.Count; i++)
            {
                previous_round_set.Add(country_to_number[names_previous_round.ElementAt(i).Value]);
            }

            int[] random_array = createRandomArray(names_previous_round.Count);

            while (!previous_round_set.Overlaps(random_array))
            {
                random_array = createRandomArray(names_previous_round.Count);
            }
        }
    }
}