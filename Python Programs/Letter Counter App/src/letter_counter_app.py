# ------------------------------------ functions --------------------------- #

# this is our welcome message for the app
def print_welcome():
    print()
    print("Welcome to the letter counter app")
    print()


# we ask for the user's name
def input_user_name():
    name = input("What is your name: ")
    # converts the input string to proper case or title case. 
    # that is, all words begin with uppercase and the rest are lowercase.
    name = name.title().strip()
    return name


# we welcome the user with a message here with their name
def print_hello_name(name):
    output_string = f"Hello, {name}!"
    print()
    print(output_string)


# this message will let a user know what the app does
def print_app_explanation():
    print()
    print("I will count the number of times that a specific letter occurs in "\
          "a message.")
    print()


# this function will ask the user for a message
def input_message():
    message = input("Please enter a message: ")
    return message


# this function will ask the user which letter they'd like to count
# in their message
def input_letter_to_count():
    letter = input("Which letter would you like to count the occurrences of: ")
    return letter


# this function will return a count of occurences of a letter
# inside of a message that's supplied
def count_letter_occurence(letter, message):
    letter = letter.lower()
    message = message.lower()
    count_of_letters = message.count(letter)
    return count_of_letters


# this is the output message of the program
# it will tell the user the letter occurences in their message
def print_app_output(name, letter, count_of_letters):
    output_string = f"{name}, your message has {count_of_letters} {letter}'s "\
                        "in it."
    print()
    print(output_string)
    print()


# ------------------------------------ main --------------------------------- #
# this the main app function
def main():
    print_welcome()

    name = input_user_name()

    print_hello_name(name)
    
    print_app_explanation()

    message = input_message()

    letter = input_letter_to_count()

    count_of_letters = count_letter_occurence(letter, message)

    print_app_output(name, letter, count_of_letters)


# ------------------------------------ run program ---------------------------#
if __name__ == "__main__":
    main()
