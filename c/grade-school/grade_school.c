#include "grade_school.h"
#include <stdlib.h>
#include <string.h>
//
#include <stdio.h>
/*
 *
 typedef struct {
 size_t count;
 student_t students[MAX_STUDENTS];
 } roster_t;


 typedef struct {
 uint8_t grade;
 char name[MAX_NAME_LENGTH];
 } student_t;
 * */
int compare_grade(void *a, void *b);
int compare_name(void *a, void *b);

void init_roster(roster_t *rost)
{
        rost->count = 0;
        student_t s;
        s.grade = 0;
        for (int i = 0; i  < MAX_STUDENTS; i++){
                rost->students[i] = s;
        }

}

int compare_grade(void *a, void *b)
{
        const student_t *stu1 = (const student_t *)(a);
        const student_t *stu2 = (const  student_t *)(b);
        return (stu1->grade - stu2->grade);
}
int compare_name(void *a, void *b)
{
        const student_t *stu1 = (const student_t *)(a);
        const student_t *stu2 = (const  student_t *)(b);
        return strcmp(stu1->name, stu2->name);
}

uint8_t add_student(roster_t *rost, char * name, uint8_t grade)
{
        if (rost->count < MAX_STUDENTS){
                for (int i = 0; i  < MAX_STUDENTS; i++){
                        student_t s = rost->students[i];
                        if (s.grade == grade) {
                                if (strcmp(s.name, name) == 0)
                                        return 0;
                        }
                }
                student_t st;
                st.grade = grade;
                strcpy(st.name, name);
                rost->students[rost->count] = st;
                rost->count++;
        } else {
                return 0;
        }
        return 1;
}

roster_t get_grade(roster_t *rost, uint8_t desired)
{
        roster_t r;
        r.count = rost->count;
        // r.students = rost.students;
        printf("%d\n", desired);
        memcpy(r.students, rost->students, sizeof(rost->students));
        return r;
}
