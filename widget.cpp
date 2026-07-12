#include "widget.h"
#include "./ui_widget.h"
#include "ast.h"

Widget::Widget(QWidget *parent)
    : QWidget(parent)
    , ui(new Ui::Widget)
{
    ui->setupUi(this);
    this->setWindowTitle("Calculator");
}

Widget::~Widget()
{
    delete ui;
}

void Widget::update()
{
    result.clear();
    result = input;

    result.replace("a", "＋");
    result.replace("s", "−");
    result.replace("m", "×");
    result.replace("d", "÷");
    result.replace("l", "(");
    result.replace("r", ")");

    auto value = QString();
    try {
        auto expr = AST::parse(input);
        value = QString::number(expr->eval());
        result.append("\n=");

    } catch (const std::runtime_error &e) {
    }

    result.push_back(value);

    ui->textBrowser->setText(result);
}

#define NUMBER_BUTTON(CHAR) \
    void Widget::on_pushButton_##CHAR##_clicked() \
    { \
        input.push_back(#CHAR[0]); \
        update(); \
    }

NUMBER_BUTTON(0)
NUMBER_BUTTON(1)
NUMBER_BUTTON(2)
NUMBER_BUTTON(3)
NUMBER_BUTTON(4)
NUMBER_BUTTON(5)
NUMBER_BUTTON(6)
NUMBER_BUTTON(7)
NUMBER_BUTTON(8)
NUMBER_BUTTON(9)

NUMBER_BUTTON(a)
NUMBER_BUTTON(s)
NUMBER_BUTTON(m)
NUMBER_BUTTON(d)
NUMBER_BUTTON(l)
NUMBER_BUTTON(r)

void Widget::on_pushButton_C_clicked()
{
    input.clear();
    update();
}

void Widget::on_pushButton_back_clicked()
{
    if (!input.isEmpty())
        input.chop(1);

    update();
}

void Widget::on_pushButton_eq_clicked()
{
    try {
        auto expr = AST::parse(input);
        ui->textBrowser->setText(QString::number(expr->eval()));

    } catch (const std::runtime_error &e) {
        ui->textBrowser->setText("错误");
    }
}
