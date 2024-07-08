import {Router} from 'express';
import { getTasks, createTask, getTaskById, deleteTask, updateTask } from '../controllers/taskController';

const router = Router();

router.get('/get', getTasks);
router.post('/create', createTask);
router.put('/update/:id', updateTask);
router.delete('/delete/:id', deleteTask);
router.get('/get_task/:id', getTaskById);

export default router