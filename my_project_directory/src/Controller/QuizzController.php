<?php

// src/Controller/QuizzController.php
namespace App\Controller;

use App\Form\QuizzType;
use App\Entity\Categorie;
use App\Repository\CategorieRepository;
use Psr\Log\LoggerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpFoundation\Session\SessionInterface;
use Symfony\Component\Routing\Annotation\Route;

#[Route('/quizz')]
class QuizzController extends AbstractController
{
    private $logger;

    public function __construct(LoggerInterface $logger)
    {
        $this->logger = $logger;
    }

    #[Route('/{id}', name: 'app_quizz', methods: ['GET', 'POST'])]
    public function index(CategorieRepository $categorieRepository, Request $request, SessionInterface $session, int $id): Response
    {
        $categorie = $categorieRepository->find($id);

        if (!$categorie) {
            $this->logger->error('Category not found: ' . $id);
            throw $this->createNotFoundException('La catégorie n\'existe pas');
        }

        $questions = $categorie->getQuestions()->toArray();
        $form = $this->createForm(QuizzType::class, null, ['questions' => $questions]);
        $form->handleRequest($request);

        if ($form->isSubmitted() && $form->isValid()) {
            $this->logger->info('Form is valid');
            $score = 0;
            $selectedResponses = $form->getData();

            // Log form data
            $this->logger->info('Form data: ' . json_encode($selectedResponses));

            foreach ($questions as $question) {
                $selectedResponseId = $selectedResponses['question_' . $question->getId()];
                $correctResponses = [];

                foreach ($question->getReponses() as $reponse) {
                    if ($reponse->isReponseExpected()) {
                        $correctResponses[] = $reponse->getId();
                    }
                }

                if (in_array($selectedResponseId, $correctResponses)) {
                    $score++;
                }
            }

            $session->set('score', $score);
            $session->set('total', count($questions));
            $session->set('name', $categorie->getName());
            $this->logger->info('Score calculated: ' . $score);

            return $this->redirectToRoute('app_quizz_result', [
                'id' => $id,
            ]);
        } else {
            $this->logger->info('Form is not valid or not submitted');
        }

        return $this->render('quizz/index.html.twig', [
            'currentRoute' => 'app_quizz',
            'categorie' => $categorie,
            'form' => $form->createView(),
        ]);
    }

    #[Route('/result/{id}', name: 'app_quizz_result', methods: ['GET'])]
    public function result(SessionInterface $session, int $id): Response
    {
        $score = $session->get('score');
        $total = $session->get('total');
        $name = $session->get('name');

        if ($score === null || $total === null) {
            $this->logger->error('Score or total not found in session');
            throw $this->createNotFoundException('Score or total not found in session');
        }

        return $this->render('quizz/result.html.twig', [
            'score' => $score,
            'total' => $total,
            'id' => $id,
            'name' => $name,
        ]);
    }
}
