/********************************************************************************
** Form generated from reading UI file 'widget.ui'
**
** Created by: Qt User Interface Compiler version 6.11.1
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_WIDGET_H
#define UI_WIDGET_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QTextBrowser>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_Widget
{
public:
    QVBoxLayout *verticalLayout;
    QTextBrowser *textBrowser;
    QGridLayout *gridLayout_2;
    QPushButton *pushButton_back;
    QPushButton *pushButton_a;
    QPushButton *pushButton_C;
    QPushButton *pushButton_r;
    QPushButton *pushButton_eq;
    QPushButton *pushButton_m;
    QPushButton *pushButton_3;
    QPushButton *pushButton_6;
    QPushButton *pushButton_1;
    QPushButton *pushButton_l;
    QPushButton *pushButton_s;
    QPushButton *pushButton_9;
    QPushButton *pushButton_8;
    QPushButton *pushButton_d;
    QPushButton *pushButton_0;
    QPushButton *pushButton_5;
    QPushButton *pushButton_7;
    QPushButton *pushButton_2;
    QPushButton *pushButton_4;

    void setupUi(QWidget *Widget)
    {
        if (Widget->objectName().isEmpty())
            Widget->setObjectName("Widget");
        Widget->resize(800, 600);
        verticalLayout = new QVBoxLayout(Widget);
        verticalLayout->setObjectName("verticalLayout");
        textBrowser = new QTextBrowser(Widget);
        textBrowser->setObjectName("textBrowser");
        QFont font;
        font.setPointSize(30);
        font.setBold(true);
        textBrowser->setFont(font);

        verticalLayout->addWidget(textBrowser);

        gridLayout_2 = new QGridLayout();
        gridLayout_2->setObjectName("gridLayout_2");
        pushButton_back = new QPushButton(Widget);
        pushButton_back->setObjectName("pushButton_back");
        QSizePolicy sizePolicy(QSizePolicy::Policy::Expanding, QSizePolicy::Policy::Expanding);
        sizePolicy.setHorizontalStretch(0);
        sizePolicy.setVerticalStretch(0);
        sizePolicy.setHeightForWidth(pushButton_back->sizePolicy().hasHeightForWidth());
        pushButton_back->setSizePolicy(sizePolicy);
        QFont font1;
        font1.setPointSize(20);
        font1.setBold(true);
        pushButton_back->setFont(font1);

        gridLayout_2->addWidget(pushButton_back, 0, 3, 1, 1);

        pushButton_a = new QPushButton(Widget);
        pushButton_a->setObjectName("pushButton_a");
        sizePolicy.setHeightForWidth(pushButton_a->sizePolicy().hasHeightForWidth());
        pushButton_a->setSizePolicy(sizePolicy);
        pushButton_a->setFont(font1);

        gridLayout_2->addWidget(pushButton_a, 0, 1, 1, 1);

        pushButton_C = new QPushButton(Widget);
        pushButton_C->setObjectName("pushButton_C");
        sizePolicy.setHeightForWidth(pushButton_C->sizePolicy().hasHeightForWidth());
        pushButton_C->setSizePolicy(sizePolicy);
        pushButton_C->setFont(font1);

        gridLayout_2->addWidget(pushButton_C, 0, 0, 1, 1);

        pushButton_r = new QPushButton(Widget);
        pushButton_r->setObjectName("pushButton_r");
        sizePolicy.setHeightForWidth(pushButton_r->sizePolicy().hasHeightForWidth());
        pushButton_r->setSizePolicy(sizePolicy);
        pushButton_r->setFont(font1);

        gridLayout_2->addWidget(pushButton_r, 7, 2, 1, 1);

        pushButton_eq = new QPushButton(Widget);
        pushButton_eq->setObjectName("pushButton_eq");
        sizePolicy.setHeightForWidth(pushButton_eq->sizePolicy().hasHeightForWidth());
        pushButton_eq->setSizePolicy(sizePolicy);
        pushButton_eq->setFont(font1);

        gridLayout_2->addWidget(pushButton_eq, 5, 3, 3, 1);

        pushButton_m = new QPushButton(Widget);
        pushButton_m->setObjectName("pushButton_m");
        sizePolicy.setHeightForWidth(pushButton_m->sizePolicy().hasHeightForWidth());
        pushButton_m->setSizePolicy(sizePolicy);
        pushButton_m->setFont(font1);

        gridLayout_2->addWidget(pushButton_m, 2, 3, 1, 1);

        pushButton_3 = new QPushButton(Widget);
        pushButton_3->setObjectName("pushButton_3");
        sizePolicy.setHeightForWidth(pushButton_3->sizePolicy().hasHeightForWidth());
        pushButton_3->setSizePolicy(sizePolicy);
        pushButton_3->setFont(font1);

        gridLayout_2->addWidget(pushButton_3, 5, 2, 1, 1);

        pushButton_6 = new QPushButton(Widget);
        pushButton_6->setObjectName("pushButton_6");
        sizePolicy.setHeightForWidth(pushButton_6->sizePolicy().hasHeightForWidth());
        pushButton_6->setSizePolicy(sizePolicy);
        pushButton_6->setFont(font1);

        gridLayout_2->addWidget(pushButton_6, 4, 2, 1, 1);

        pushButton_1 = new QPushButton(Widget);
        pushButton_1->setObjectName("pushButton_1");
        sizePolicy.setHeightForWidth(pushButton_1->sizePolicy().hasHeightForWidth());
        pushButton_1->setSizePolicy(sizePolicy);
        pushButton_1->setFont(font1);

        gridLayout_2->addWidget(pushButton_1, 5, 0, 1, 1);

        pushButton_l = new QPushButton(Widget);
        pushButton_l->setObjectName("pushButton_l");
        sizePolicy.setHeightForWidth(pushButton_l->sizePolicy().hasHeightForWidth());
        pushButton_l->setSizePolicy(sizePolicy);
        pushButton_l->setFont(font1);

        gridLayout_2->addWidget(pushButton_l, 7, 0, 1, 1);

        pushButton_s = new QPushButton(Widget);
        pushButton_s->setObjectName("pushButton_s");
        sizePolicy.setHeightForWidth(pushButton_s->sizePolicy().hasHeightForWidth());
        pushButton_s->setSizePolicy(sizePolicy);
        pushButton_s->setFont(font1);

        gridLayout_2->addWidget(pushButton_s, 0, 2, 1, 1);

        pushButton_9 = new QPushButton(Widget);
        pushButton_9->setObjectName("pushButton_9");
        sizePolicy.setHeightForWidth(pushButton_9->sizePolicy().hasHeightForWidth());
        pushButton_9->setSizePolicy(sizePolicy);
        pushButton_9->setFont(font1);

        gridLayout_2->addWidget(pushButton_9, 2, 2, 1, 1);

        pushButton_8 = new QPushButton(Widget);
        pushButton_8->setObjectName("pushButton_8");
        sizePolicy.setHeightForWidth(pushButton_8->sizePolicy().hasHeightForWidth());
        pushButton_8->setSizePolicy(sizePolicy);
        pushButton_8->setFont(font1);

        gridLayout_2->addWidget(pushButton_8, 2, 1, 1, 1);

        pushButton_d = new QPushButton(Widget);
        pushButton_d->setObjectName("pushButton_d");
        sizePolicy.setHeightForWidth(pushButton_d->sizePolicy().hasHeightForWidth());
        pushButton_d->setSizePolicy(sizePolicy);
        pushButton_d->setFont(font1);

        gridLayout_2->addWidget(pushButton_d, 4, 3, 1, 1);

        pushButton_0 = new QPushButton(Widget);
        pushButton_0->setObjectName("pushButton_0");
        sizePolicy.setHeightForWidth(pushButton_0->sizePolicy().hasHeightForWidth());
        pushButton_0->setSizePolicy(sizePolicy);
        pushButton_0->setFont(font1);

        gridLayout_2->addWidget(pushButton_0, 7, 1, 1, 1);

        pushButton_5 = new QPushButton(Widget);
        pushButton_5->setObjectName("pushButton_5");
        sizePolicy.setHeightForWidth(pushButton_5->sizePolicy().hasHeightForWidth());
        pushButton_5->setSizePolicy(sizePolicy);
        pushButton_5->setFont(font1);

        gridLayout_2->addWidget(pushButton_5, 4, 1, 1, 1);

        pushButton_7 = new QPushButton(Widget);
        pushButton_7->setObjectName("pushButton_7");
        sizePolicy.setHeightForWidth(pushButton_7->sizePolicy().hasHeightForWidth());
        pushButton_7->setSizePolicy(sizePolicy);
        pushButton_7->setFont(font1);

        gridLayout_2->addWidget(pushButton_7, 2, 0, 1, 1);

        pushButton_2 = new QPushButton(Widget);
        pushButton_2->setObjectName("pushButton_2");
        sizePolicy.setHeightForWidth(pushButton_2->sizePolicy().hasHeightForWidth());
        pushButton_2->setSizePolicy(sizePolicy);
        pushButton_2->setFont(font1);

        gridLayout_2->addWidget(pushButton_2, 5, 1, 1, 1);

        pushButton_4 = new QPushButton(Widget);
        pushButton_4->setObjectName("pushButton_4");
        sizePolicy.setHeightForWidth(pushButton_4->sizePolicy().hasHeightForWidth());
        pushButton_4->setSizePolicy(sizePolicy);
        pushButton_4->setFont(font1);

        gridLayout_2->addWidget(pushButton_4, 4, 0, 1, 1);


        verticalLayout->addLayout(gridLayout_2);


        retranslateUi(Widget);

        QMetaObject::connectSlotsByName(Widget);
    } // setupUi

    void retranslateUi(QWidget *Widget)
    {
        Widget->setWindowTitle(QCoreApplication::translate("Widget", "Widget", nullptr));
        pushButton_back->setText(QCoreApplication::translate("Widget", "\342\214\253", nullptr));
        pushButton_a->setText(QCoreApplication::translate("Widget", "\357\274\213", nullptr));
        pushButton_C->setText(QCoreApplication::translate("Widget", "C", nullptr));
        pushButton_r->setText(QCoreApplication::translate("Widget", ")", nullptr));
        pushButton_eq->setText(QCoreApplication::translate("Widget", "=", nullptr));
        pushButton_m->setText(QCoreApplication::translate("Widget", "\303\227", nullptr));
        pushButton_3->setText(QCoreApplication::translate("Widget", "3", nullptr));
        pushButton_6->setText(QCoreApplication::translate("Widget", "6", nullptr));
        pushButton_1->setText(QCoreApplication::translate("Widget", "1", nullptr));
        pushButton_l->setText(QCoreApplication::translate("Widget", "(", nullptr));
        pushButton_s->setText(QCoreApplication::translate("Widget", "\342\210\222", nullptr));
        pushButton_9->setText(QCoreApplication::translate("Widget", "9", nullptr));
        pushButton_8->setText(QCoreApplication::translate("Widget", "8", nullptr));
        pushButton_d->setText(QCoreApplication::translate("Widget", "\303\267", nullptr));
        pushButton_0->setText(QCoreApplication::translate("Widget", "0", nullptr));
        pushButton_5->setText(QCoreApplication::translate("Widget", "5", nullptr));
        pushButton_7->setText(QCoreApplication::translate("Widget", "7", nullptr));
        pushButton_2->setText(QCoreApplication::translate("Widget", "2", nullptr));
        pushButton_4->setText(QCoreApplication::translate("Widget", "4", nullptr));
    } // retranslateUi

};

namespace Ui {
    class Widget: public Ui_Widget {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_WIDGET_H
