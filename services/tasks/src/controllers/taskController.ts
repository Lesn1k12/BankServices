import { Request, Response } from "express";
import { title } from "process";
import Task from "../models/Tasks"; 

export const createTask = async (req: Request, res: Response) => {
    try {
        const newTask = new Task(
            {
                title: req.body.title,
                description: req.body.description,
            }
        );
        const task = await newTask.save();
        res.status(201).send(task);
    } catch (err) {
        res.status(400).send(err);
    }
}

export const getTasks = async (req: Request, res: Response) => {
    try{
        const tasks = await Task.find();
        res.status(200).send(tasks);
    }
    catch(err){
        res.status(400).send(err);
    }
}

export const getTaskById = async (req: Request, res: Response) => {
    try{
        const task = await Task.findById(req.params.id);
        res.status(200).send(task);
    }
    catch(err){
        res.status(400).send(err);
    }
}

export const updateTask = async (req: Request, res: Response) => {
    try{
        const task = await Task.findByIdAndUpdate(req.params.id, req.body, { new: true });
        if (!task){
            res.status(404).send("Task not found");
        }
        res.status(200).send(task);
    }
    catch(err){
        res.status(400).send(err);
    }
}

export const deleteTask = async (req: Request, res: Response) => {
    try{
        const task = await Task.findByIdAndDelete(req.params.id);
        if (!task){
            res.status(404).send("Task not found");
        }
        res.status(200).send(task);
    }
    catch(err){
        res.status
    }
}