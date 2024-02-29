# Class to wrap the different tokens we'll be using
from enum import Enum


class TokenType(Enum):
    identifier = 1
    whitespace = 2
    void = 3
    end = 4


class Token:
    def __init__(self, t, l):
        self.type = t
        self.lexeme = l


# Lexer class ... an instance will be available to the parser
class Lexer:
    def __init__(self):
        self.lexeme_list = ["_", "letter", "digit", "ws", "other"]
        self.states_list = [0, 1, 2]
        self.states_accp = [1, 2]

        self.rows = len(self.states_list)
        self.cols = len(self.lexeme_list)

        # Let's take integer -1 to represent the error state for this DFA
        self.Tx = [[-1 for j in range(self.cols)] for i in range(self.rows)]
        self.InitialiseTxTable()

    def InitialiseTxTable(self):
        # Update Tx to represent the state transition function of the DFA
        # Variables
        self.Tx[0][self.lexeme_list.index("letter")] = 1
        self.Tx[0][self.lexeme_list.index("_")] = 1
        self.Tx[1][self.lexeme_list.index("letter")] = 1
        self.Tx[1][self.lexeme_list.index("digit")] = 1

        # White Space
        self.Tx[0][self.lexeme_list.index("ws")] = 2
        self.Tx[2][self.lexeme_list.index("ws")] = 2

        for row in self.Tx:
            print(row)

    def AcceptingStates(self, state):
        try:
            self.states_accp.index(state)
            return True
        except ValueError:
            return False

    def GetTokenTypeByFinalState(self, state, lexeme):
        if state == 1:
            return Token(TokenType.identifier, lexeme)
        elif state == 2:
            return Token(TokenType.whitespace, lexeme)

        else:
            return "default result"

    def CatChar(self, character):
        cat = "other"
        if character.isalpha():
            cat = "letter"
        if character.isdigit():
            cat = "digit"
        if character == "_":
            cat = "_"
        if character == " ":
            cat = "ws"
        return cat

    def EndOfInput(self, src_program_str, src_program_idx):
        if src_program_idx > len(src_program_str) - 1:
            return True
        else:
            return False

    def NextChar(self, src_program_str, src_program_idx):
        if not self.EndOfInput(src_program_str, src_program_idx):
            return True, src_program_str[src_program_idx]
        else:
            return False, "."

    def NextToken(self, src_program_str, src_program_idx):
        state = 0  # initial state is 0 - check Tx
        stack = []
        lexeme = ""
        stack.append(-2)
        # insert the error state at the bottom of the stack.

        while state != -1:
            if self.AcceptingStates(state):
                stack.clear()
            stack.append(state)

            exists, character = self.NextChar(src_program_str, src_program_idx)
            lexeme += character
            if not exists:
                # print("LAST LEXEME: ", lexeme);
                break  # Break out of loop if we're at the end of the string
            src_program_idx = src_program_idx + 1

            cat = self.CatChar(character)
            state = self.Tx[state][self.lexeme_list.index(cat)]
            # print("Lexeme: ", lexeme, " => NEXT STATE: ", state, "  => CAT: ", cat, "  => CHAR:", character, "  => STACK: ", stack)

        lexeme = lexeme[
            :-1
        ]  # remove the last character added which sent the lexer to state -1

        syntax_error = False
        # rollback
        while len(stack) > 0:
            if stack[-1] == -2:  # report a syntax error
                syntax_error = True
                exists, character = self.NextChar(src_program_str, src_program_idx - 1)
                lexeme = character
                break

            # Pop this state if not an accepting state.
            if not self.AcceptingStates(stack[-1]):
                stack.pop()
                # print("POPPED => ", stack)
                lexeme = lexeme[:-1]

            # This is an accepting state ... return it.
            else:
                state = stack.pop()
                break

        # print("Lexeme: ", lexeme, "with state: ", state)

        if syntax_error:
            return Token(TokenType.void, lexeme), "error"

        if self.AcceptingStates(state):
            return self.GetTokenTypeByFinalState(state, lexeme), lexeme
        else:
            return Token(TokenType.void, lexeme), "Lexical Error"

    def GenerateTokens(self, src_program_str):
        print("INPUT:: " + src_program_str)
        tokens_list = []
        src_program_idx = 0
        token, lexeme = self.NextToken(src_program_str, src_program_idx)
        tokens_list.append(token)

        while token != -1:
            src_program_idx = src_program_idx + len(lexeme)
            # print ("Nxt TOKEN: ", token, " ", lexeme, "(", len(lexeme), ")  => IDX: ", src_program_idx)
            if not self.EndOfInput(src_program_str, src_program_idx):
                token, lexeme = self.NextToken(src_program_str, src_program_idx)
                tokens_list.append(token)
                if token.type == TokenType.void:
                    break
                    # A lexical error was encountered
            else:
                break  # The end of the source program

        # print("DONE!!")
        return tokens_list


lex = Lexer()
toks = lex.GenerateTokens("x          y  p _a123")

for t in toks:
    print(t.type, t.lexeme)
