course_name = []
credit = []
gpa = []

numcourse = int(input("Enter the number of courses: ").strip())

for i in range(numcourse):
    print("Enter the details for the course ")
    cm = input("Course name: ").strip()
    xcredit = int(input("Credit hour: ").strip())
    grade = int(input("Grade(0-4): ").strip())
    course_name.append(cm)
    credit.append(xcredit)
    gpa.append(grade)
totalGradePoint = 0.0
totalCreditHours = 0.0
for i in range(numcourse):
    totalGradePoint = totalGradePoint + (credit[i] * gpa[i])
    totalCreditHours = totalCreditHours + credit[i]
cgpa = totalGradePoint / totalCreditHours
print(cgpa)
