#include "grade_school.h"
#include <stdlib.h>
#include <string.h>

int compare_grade(const void *a, const void *b);
int compare_name(const void *a, const void *b);
int roster_has_student(roster_t *roster, char * name);

int compare_grade(const void *a, const void *b)
{
        uint8_t grade1 = ((const student_t *)a)->grade;
        uint8_t grade2 = ((const  student_t *)b)->grade;
        if (grade2 == 255) return -1;
        return (grade1 - grade2);
}

int compare_name(const void *a, const void *b)
{
        const char *n1 = ((const student_t *)a)->name;
        const char *n2 = ((const student_t *)b)->name;
        return strcmp(n1, n2);
}

int roster_has_student(roster_t *roster, char * name)
{
        int res = 0;
        for (int i = 0; i  < MAX_STUDENTS; i++){
                student_t s = roster->students[i];
                int cmp = strcmp(s.name, name);
                if (cmp == 0){
                        return 1;
                }

        }
        return res;
}

void init_roster(roster_t *rost)
{
        rost->count = 0;

        student_t s;
        s.grade = 255;
        for (int i = 0; i <  MAX_NAME_LENGTH; i++){
                s.name[i] = '\0';
        }
        for (int i = 0; i  < MAX_STUDENTS; i++){
                rost->students[i] = s;
        }

}


uint8_t add_student(roster_t *rost, char * name, uint8_t grade)
{
        uint8_t res = 0;
        if (!roster_has_student(rost, name)) {
                student_t st;
                st.grade = grade;
                strcpy(st.name, name);
                rost->students[rost->count] = st;
                rost->count++;
                res++;
                qsort(rost->students, MAX_STUDENTS,
                                sizeof(student_t), compare_name);
                qsort(rost->students, MAX_STUDENTS,
                                sizeof(student_t), compare_grade);
        }
        return res;
}

roster_t get_grade(roster_t *rost, uint8_t desired)
{
        roster_t r;
        r.count = 0;
        for (int i = 0; i  < MAX_STUDENTS; i++){
                student_t s = rost->students[i];
                if (s.grade == desired) {
                        r.students[r.count] = s;
                        r.count++;
                }

        }
        return r;
}
