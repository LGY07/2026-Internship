#ifndef WIDGET_H
#define WIDGET_H

#include <QTextBrowser>
#include <QWidget>

QT_BEGIN_NAMESPACE
namespace Ui {
class Widget;
}
QT_END_NAMESPACE

class Widget : public QWidget
{
    Q_OBJECT

public:
    explicit Widget(QWidget *parent = nullptr);
    ~Widget() override;

private slots:
    void on_pushButton_0_clicked();
    void on_pushButton_1_clicked();
    void on_pushButton_2_clicked();
    void on_pushButton_3_clicked();
    void on_pushButton_4_clicked();
    void on_pushButton_5_clicked();
    void on_pushButton_6_clicked();
    void on_pushButton_7_clicked();
    void on_pushButton_8_clicked();
    void on_pushButton_9_clicked();

    void on_pushButton_a_clicked();
    void on_pushButton_s_clicked();
    void on_pushButton_m_clicked();
    void on_pushButton_d_clicked();

    void on_pushButton_l_clicked();
    void on_pushButton_r_clicked();

    void on_pushButton_C_clicked();
    void on_pushButton_back_clicked();
    void on_pushButton_eq_clicked();

private:
    Ui::Widget *ui;

    QString input;
    QString result;

    void update();
};
#endif // WIDGET_H
