from random import randint

def random_call(random_array, length):
    ran = randint(1, length)
    if ran not in random_array:
        random_array.append(ran)
    return random_array

def create_random_array(length):
    random_array = []
    while len(random_array) < length:
        random_array = random_call(random_array, length)
    return random_array

def main():
    names_previous_round = {
        "Ben": "Turkey",
        "Casper": "Germany",
        "Niels": "Austria",
        "Koen": "Italy",
        "Lock": "England",
        "Jan-Jan": "Russia",
        "Wouter": "France"
    }
    
    country_to_number = {
        "France": 1,
        "Russia": 2,
        "Italy": 3,
        "Germany": 4,
        "England": 5,
        "Austria": 6,
        "Turkey": 7
    }
    
    # Create a set of previously used countries as numbers
    previous_round_set = {country_to_number[country] for country in names_previous_round.values()}
    
    # Create a random array of unique numbers
    random_array = create_random_array(len(names_previous_round))
    
    # Ensure the random array does not contain any previously used numbers
    while not set(random_array).isdisjoint(previous_round_set):
        random_array = create_random_array(len(names_previous_round))
    
    # Create a reverse mapping from numbers back to countries
    number_to_country = {v: k for k, v in country_to_number.items()}
    
    # Create a dictionary mapping names to their corresponding countries
    name_dict = {}
    names = list(names_previous_round.keys())
    
    for i in range(len(names)):
        # Convert the random number back to the corresponding country
        country_number = random_array[i]
        country_name = number_to_country[country_number]
        name_dict[names[i]] = country_name
        
    # Print the results
    for name, country in name_dict.items():
        print(f"name: {name}, country: {country}")

if __name__ == "__main__":
    main()
