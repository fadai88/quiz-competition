const mongoose = require('mongoose');
require('dotenv').config();

mongoose.connect(process.env.MONGODB_URI, { useNewUrlParser: true, useUnifiedTopology: true })
    .then(() => console.log('Connected to MongoDB'))
    .catch(err => console.error('Could not connect to MongoDB', err));

const QuizSchema = new mongoose.Schema({
    question: String,
    options: [String],
    correctAnswer: Number
});

const Quiz = mongoose.model('Quiz', QuizSchema);

async function verifyMigration() {
    try {
        const count = await Quiz.countDocuments();
        console.log(`Total number of questions in MongoDB: ${count}`);

        const sampleQuestions = await Quiz.find().limit(5);
        console.log('\nSample Questions:');
        sampleQuestions.forEach((q, index) => {
            console.log(`\n${index + 1}. ${q.question}`);
            q.options.forEach((option, i) => {
                console.log(`   ${String.fromCharCode(65 + i)}) ${option}${i === q.correctAnswer ? ' (Correct)' : ''}`);
            });
        });

    } catch (error) {
        console.error('Error verifying migration:', error);
    } finally {
        mongoose.connection.close();
    }
}

verifyMigration();