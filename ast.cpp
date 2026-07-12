#include "ast.h"

#include <stdexcept>

class AST::Parser
{
public:
    explicit Parser(const QString &s)
        : str(s)
    {}

    std::unique_ptr<AST> parseExpr()
    {
        auto node = parseTerm();

        while (true) {
            char c = peek();

            if (c != 'a' && c != 's')
                break;

            get();

            auto right = parseTerm();

            node = std::unique_ptr<AST>(new AST(c, std::move(node), std::move(right)));
        }

        return node;
    }

private:
    QString str;
    int pos = 0;

    std::unique_ptr<AST> parseTerm()
    {
        auto node = parseFactor();

        while (true) {
            char c = peek();

            if (c != 'm' && c != 'd')
                break;

            get();

            auto right = parseFactor();

            node = std::unique_ptr<AST>(new AST(c, std::move(node), std::move(right)));
        }

        return node;
    }

    std::unique_ptr<AST> parseFactor()
    {
        char c = peek();

        if (c == 'l') {
            get();

            auto node = parseExpr();

            if (get() != 'r')
                throw std::runtime_error("missing right bracket");

            return node;
        }

        return parseNumber();
    }

    std::unique_ptr<AST> parseNumber()
    {
        int number = 0;

        bool found = false;

        while (peek() >= '0' && peek() <= '9') {
            found = true;

            number = number * 10 + (get() - '0');
        }

        if (!found)
            throw std::runtime_error("invalid number");

        return std::unique_ptr<AST>(new AST(number));
    }

    char peek()
    {
        if (pos >= str.size())
            return '\0';

        return str[pos].toLatin1();
    }

    char get()
    {
        char c = peek();

        if (c != '\0')
            pos++;

        return c;
    }
};

AST::AST(double v)
    : type(Type::Number)
    , value(v)
{}

AST::AST(char op, std::unique_ptr<AST> l, std::unique_ptr<AST> r)
    : type(Type::Binary)
    , op(op)
    , left(std::move(l))
    , right(std::move(r))
{}

std::unique_ptr<AST> AST::parse(const QString &expr)
{
    Parser parser(expr);

    return parser.parseExpr();
}

double AST::eval() const
{
    if (type == Type::Number)
        return value;

    double l = left->eval();
    double r = right->eval();

    switch (op) {
    case 'a':
        return l + r;

    case 's':
        return l - r;

    case 'm':
        return l * r;

    case 'd':

        if (r == 0)
            throw std::runtime_error("divide zero");

        return l / r;
    }

    throw std::runtime_error("unknown operator");
}