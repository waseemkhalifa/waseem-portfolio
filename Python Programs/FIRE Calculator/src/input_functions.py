def input_current_holdings(classification:str) -> float:
    acceptable_input:bool = False

    while acceptable_input == False:
        current_holdings:str = input(f"Enter your current {classification} holdings: ")
        
        try:
            current_holdings:float = float(current_holdings)
            acceptable_input = True
        except ValueError:
            print(f"{current_holdings} is an unacceptable input, please enter a numerical value")
    
    return current_holdings



def input_deposits(classification:str) -> float:
    acceptable_input:bool = False

    while acceptable_input == False:
        deposit:str = input(f"Enter your monthly {classification} contributions: ")
        
        try:
            deposit:float = float(deposit)
            acceptable_input = True
        except ValueError:
            print(f"{deposit} is an unacceptable input, please enter a numerical value")
    
    return deposit



def input_age(classification:str) -> float:
    acceptable_input:bool = False

    while acceptable_input == False:
        age:str = input(f"Enter your {classification} age (0-100): ")
        
        try:
            age:float = float(age)
            acceptable_input = True
        except ValueError:
            print(f"{age} is an unacceptable input, please enter a numerical value")

    return age



def input_assumed_yearly_growth(classification:str) -> float:
    acceptable_input:bool = False

    while acceptable_input == False:
        assumed_yearly_growth:str = input("Enter assumed yearly growth (0.00-100.00) for your {classification}: ")
        
        try:
            assumed_yearly_growth:float = float(assumed_yearly_growth)
            acceptable_input = True
        except ValueError:
            print(f"{assumed_yearly_growth} is an unacceptable input, please enter a numerical value")
            
    return assumed_yearly_growth
