/*
|--------------------------------------------------------------------------
| Routes file
|--------------------------------------------------------------------------
|
| The routes file is used for defining the HTTP routes.
|
*/
import router from '@adonisjs/core/services/router'
import StudentsController from '#controllers/students_controller'

//gets students
router.get('/students', [StudentsController, 'index'])

//gets a specific student using id
router.get('/students/:id', [StudentsController, 'show'])

//add a student
router.post('/students', [StudentsController, 'store'])
