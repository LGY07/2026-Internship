#ifndef AST_H
#define AST_H

#include <QString>
#include <memory>

class AST
{
public:
    enum class Type { Number, Binary };

    static std::unique_ptr<AST> parse(const QString &expr);

    double eval() const;

private:
    Type type;

    // Number
    double value = 0;

    // Binary
    char op = 0;
    std::unique_ptr<AST> left;
    std::unique_ptr<AST> right;

    explicit AST(double value);

    AST(char op, std::unique_ptr<AST> left, std::unique_ptr<AST> right);

    class Parser;

    friend class Parser;
};

#endif // AST_H
