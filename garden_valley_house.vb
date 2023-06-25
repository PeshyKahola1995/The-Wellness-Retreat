Public Class RelaxWellnessProgram

Public Sub Main()
    'Create a variable to store user's name
    Dim strName As String
    
    
    'Prompt for user's name
    Console.WriteLine("Please enter your name:")
    strName = Console.ReadLine()
    
    'Display welcome message
    Console.WriteLine("Welcome to RelaxWellness Program, " & strName & "!")
    Console.WriteLine("This program will provide a relaxing and rejuvenating break for people seeking to recharge their batteries and renew their spirit.")
    
    'Call relaxing activities methods
    Call YogaExercise(strName)
    Call ReadingActivity(strName)
    
    'Call meditation method
    Call MindfulMeditation(strName)
    Call TakeANap(strName)
    Call OutdoorWalk(strName)
    
    Console.WriteLine("By now you should be feeling relaxed and rejuvenated " & strName & "!")
End Sub

Public Sub YogaExercise(ByVal Name As String)
    Console.WriteLine("Let's start the relaxation by doing some yoga exercises " & Name & "!")
End Sub

Public Sub ReadingActivity(ByVal Name As String)
    Console.WriteLine("Now let's do some reading activity " & Name & ".")
    Console.WriteLine("Pick up a book and read for 30 minutes to help your brain to relax.")
End Sub

Public Sub MindfulMeditation(ByVal Name As String)
    Console.WriteLine("After the reading activity, let's do a mindful meditation " & Name & ".")
    Console.WriteLine("Sit in a comfortable position, close your eyes, and take some deep breaths.")
End Sub

Public Sub TakeANap(ByVal Name As String)
    Console.WriteLine("After the mindful meditation, let's take a nap " & Name & "!")
    Console.WriteLine("Set a timer for 15 minutes and lie down for a peaceful and rejuvenating rest.")
End Sub

Public Sub OutdoorWalk(ByVal Name As String)
    Console.WriteLine("The last activity to complete your relaxation session is an outdoor walk " & Name & ".")
    Console.WriteLine("Go outside, breathe the fresh air, and enjoy the beauty of nature.")
End Sub

End Class