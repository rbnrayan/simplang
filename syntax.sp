Main Begin

    # Number
    Int8 NumberTen 10

    # Other types
    Bool Flag True
    Char ch 'a'

    # String
    String Str1 "hello, "
    String Str2 "world! "

    # Printf with formated string
    Printf "%s%s", Str1, Str2

    # Change a variable value
    Set NumberTen 8

    # If
    If Eq NumberTen, 8 Then
      Printf "NumberTen is equal to 8"
    Else
      Printf "NumberTen is NOT equal to 8"
    End If

    # Function call
    Printf "%i", (Add, 5, NumberTen)

    # Loop
    Int8 i 10
    While Gt i, 0 Then
        Set i i-1
    While End

    # For loop
    For K, V In Pair({ key1 = value1, key2 = value2 }) Do
        Printf "[%s] => %s", K, V
    For End

Main End

# Last expr is returned
Function Add (Int8 a, Int8 b) Returns Int8
Add Begin
    a + b
Add End

# Comment
# ------------------- #
#      For later      #
# ------------------- #

#===
  Multi-line comment
===#

#===
# Struct
Struct Person Begin
    String Name
    u_Int8 Age
Struct Person End

# Struct impl like Rust
Person::Impl Begin
    Function New (String name, u_Int8 Age) Returns Person
    New Begin
        Person {
            Name = name
            Age  = age
        }
    New End

    # Self implicit type -> Person
    Function Talk (Self, String Msg)
    Talk Begin
        Printf "(%s): %s", Self.Name, Msg
    Talk End
Person::Impl End

# Enumeration
Enum State Begin
    Ready
    NotReady
Enum State End
===#
