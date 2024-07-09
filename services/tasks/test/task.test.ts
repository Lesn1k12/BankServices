import request from 'supertest';
import app from '../src/index'; 

describe('POST/tasks', () => {
    it('should create a new task', async () => {
        const response = await request(app)
            .post('/tasks/create')
            .send({ title: 'Task 1', description: 'Description 1' })
            .expect(200);

        console.log(response.body);
        expect(response.body).toHaveProperty('task');
        expect(response.body.task).toHaveProperty('name', 'Task 1');
        expect(response.body.task).toHaveProperty('description', 'Description 1');
    });

    it('should not create a new task with invalid title', async () => {
        const response = await request(app)
            .post('/tasks')
            .send({})
            .expect(400);
        expect(response.body).toHaveProperty('error', 'Invalid title');
    });
});
