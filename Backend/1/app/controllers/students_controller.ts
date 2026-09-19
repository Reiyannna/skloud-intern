import type { HttpContext } from '@adonisjs/core/http'

const students = [
  { id: 1, name: 'Anna' },
  { id: 2, name: 'John' },
]

export default class StudentsController {
    async index({ response }: HttpContext) {
    return response.json(students)
  } 

    async show({ params, response }: HttpContext) {
    const student = students.find(
      (student) => student.id === Number(params.id)
    )

    if (!student) {
      return response.status(404).json({
        message: 'Student not found',
      })
    }

    return response.json(student)
  }

    async store({ request, response }: HttpContext) {
    const { name } = request.only(['name'])

    const newStudent = {
      id: students.length + 1,
      name,
    }

    students.push(newStudent)

    return response.status(201).json(newStudent)
  }
}
