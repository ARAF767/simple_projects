#include <iostream>
#include <vector>
#include <iomanip>

using namespace std;

struct Course{
    string name;
    int creditHours;
    double grade;
};

double calculateCGPA(const vector<Course>& courses ){
    double totalGradePoint = 0.0;
    int totalCreditHours = 0;

    for(const Course& course : courses){
        totalGradePoint += course.grade * course.creditHours;
        totalCreditHours += course.creditHours;
    }
    if (totalCreditHours > 0){
        return totalGradePoint / totalCreditHours;
    }else{
        return 0.0;
    }
}
int main(){

    vector<Course> courses;

    cout << "CGPA calcuclator" << endl;

    int numcourses;

    cout << "Enter the number of courses : ";
    cin >> numcourses;

    for (int i = 0; i < numcourses; i++)
    {
        Course course;

        cout << "\n Enter the details for the course " << i +1 << ": "<< endl;
        cout << "Course Name : ";
        cin.ignore();
        getline(cin,course.name);

        cout << "Credit hours : ";
        cin >> course.creditHours;
        
        cout << "grade (0-4) : ";
        cin >> course.grade;
        
        if(course.grade < 0 || course.grade > 4){
            cout << "Inavlid grade. Please enter a grade between 0 and 4." << endl;
            i--;
            continue;
        }

        courses.push_back(course);
    }

    double cgpa = calculateCGPA(courses);

    cout << fixed << setprecision(2);
    cout << "\n You cgpa is : " << cgpa << endl;

    return 0 ;
}
