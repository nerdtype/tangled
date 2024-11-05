pragma Style_Checks (Off);

with Ada.Containers.Vectors;
with Ada.Numerics.Discrete_Random;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces.C; use Interfaces.C;
with Interfaces.C.Strings; use Interfaces.C.Strings;
with raylib;

procedure Tangled is
   Debug          : constant Boolean := False;
   Game_Width     : constant int     := 700;
   Game_Height    : constant int     := 700;
   Ball_Radius    : constant         := 22.0;
   Ball_Fill      : constant         := 15.0;
   Starting_Balls : constant         := 4;

   type Ball_Type is record
      X, Y : int; -- Location
   end record;

   subtype Range_X is int range int (Ball_Radius * 2) .. Game_Width - int (Ball_Radius * 2);
   subtype Range_Y is int range int (Ball_Radius * 2) .. Game_Height - int (Ball_Radius * 2);

   package Random_Range_X is new Ada.Numerics.Discrete_Random (Range_X);
   package Random_Range_Y is new Ada.Numerics.Discrete_Random (Range_Y);
   use Random_Range_X;
   use Random_Range_Y;
   Gx : Random_Range_X.Generator;
   Gy : Random_Range_Y.Generator;

   package Ball_Vectors is new Ada.Containers.Vectors (Natural, Ball_Type);
   use Ball_Vectors;

   type Movement_State_Type is (Standby, Moving);

   type Movement_Operation_Type is record
      Ball          : Natural;
      Mouse_X_Start : int;
      Mouse_Y_Start : int;
      Ball_X_Start  : int;
      Ball_Y_Start  : int;
   end record;

   Level_Complete       : Boolean := False;
   Balls                : Ball_Vectors.Vector;
   Movement_State       : Movement_State_Type := Standby;
   Movement_Operation   : Movement_Operation_Type;
   Clear                : Boolean;
   Level_Complete_Wave  : raylib.Wave;
   Level_Complete_Sound : raylib.Sound;

   function Intersections_Present return Boolean is
      Lines          : constant Natural := Natural (Balls.Length);
      A, B, C, D     : Ball_Type;
      D1, D2, D3, D4 : Float; --  Determinates
   begin

      for First in 1 .. Lines - 1 loop
         for Second in First + 1 .. Lines - 1 loop
            A := Balls (First - 1);
            B := Balls (First);
            C := Balls (Second - 1);
            D := Balls (Second);

            --  Calculate determinates
            D1 := Float (A.X - C.X) * Float (B.Y - C.Y) -
                  Float (A.Y - C.Y) * Float (B.X - C.X);
            D2 := Float (A.X - D.X) * Float (B.Y - D.Y) -
                  Float (A.Y - D.Y) * Float (B.X - D.X);
            D3 := Float (C.X - A.X) * Float (D.Y - A.Y) -
                  Float (C.Y - A.Y) * Float (D.X - A.X);
            D4 := Float (C.X - B.X) * Float (D.Y - B.Y) -
                  Float (C.Y - B.Y) * Float (D.X - B.X);

            if D1 * D2 < 0.0 and then D3 * D4 < 0.0 then
               --  Left side determinate is opposite sign of right side of
               --  determinant for both pairs D1, D2 and D3, D4.
               return True;
            end if;
         end loop;
      end loop;
      for First in 1 .. Lines - 1 loop
         A := Balls (0);
         B := Balls (Lines - 1);
         C := Balls (First - 1);
         D := Balls (First);

         D1 := Float (A.X - C.X) * Float (B.Y - C.Y) -
               Float (A.Y - C.Y) * Float (B.X - C.X);
         D2 := Float (A.X - D.X) * Float (B.Y - D.Y) -
               Float (A.Y - D.Y) * Float (B.X - D.X);
         D3 := Float (C.X - A.X) * Float (D.Y - A.Y) -
               Float (C.Y - A.Y) * Float (D.X - A.X);
         D4 := Float (C.X - B.X) * Float (D.Y - B.Y) -
               Float (C.Y - B.Y) * Float (D.X - B.X);

         if D1 * D2 < 0.0 and then D3 * D4 < 0.0 then
            --  An intersection is present when the left side determinate
            --  is opposite sign of right side of determinant for both
            --  pairs D1, D2 and D3, D4.
            return True;
         end if;
      end loop;
      return False;
   end Intersections_Present;

   procedure New_Level is
      Ball_Count : constant Natural := Natural (Balls.Length);
   begin
      loop
         Balls.Clear;
         for Index in 1 .. Ball_Count + 2 loop
            Balls.Append (Ball_Type'(X => Random (Gx), Y => Random (Gy)));
         end loop;
         Level_Complete := False;
         exit when Intersections_Present;
      end loop;
   end New_Level;

   procedure Update_Game is
   begin
      if Level_Complete and then Boolean (raylib.IsMouseButtonPressed (0)) then
         New_Level;
      end if;

      case Movement_State is
         when Standby =>
            if raylib.IsMouseButtonPressed (0) then
               for Index in 0 .. Natural (Balls.Length) - 1 loop
                  if Natural (abs (raylib.GetMouseX - Balls (Index).X)) < Natural (Ball_Radius) and then
                     Natural (abs (raylib.GetMouseY - Balls (Index).Y)) < Natural (Ball_Radius)
                  then
                     Movement_Operation.Ball          := Index;
                     Movement_Operation.Mouse_X_Start := raylib.GetMouseX;
                     Movement_Operation.Mouse_Y_Start := raylib.GetMouseY;
                     Movement_Operation.Ball_X_Start  := Balls (Index).X;
                     Movement_Operation.Ball_Y_Start  := Balls (Index).Y;
                     Movement_State                   := Moving;
                  end if;
               end loop;
            else
               if Intersections_Present then
                  Clear := False;
               else
                  Clear := True;
               end if;
            end if;
         when Moving =>
            if raylib.IsMouseButtonDown (0) then
               Balls (Movement_Operation.Ball).X := Movement_Operation.Ball_X_Start - (Movement_Operation.Mouse_X_Start - raylib.GetMouseX);
               Balls (Movement_Operation.Ball).Y := Movement_Operation.Ball_Y_Start - (Movement_Operation.Mouse_Y_Start - raylib.GetMouseY);
            else
               Movement_State := Standby;
               if not Intersections_Present then
                  if not Level_Complete then
                     raylib.SetSoundVolume (Level_Complete_Sound, 0.5);
                     raylib.PlaySound (Level_Complete_Sound);
                     Level_Complete := True;
                  end if;
               end if;
            end if;
      end case;
   end Update_Game;

   procedure Draw_Ball (Index : int; Ball : Ball_Type) is
   begin
      raylib.DrawCircle (Ball.X, Ball.Y, Ball_Radius, raylib.RED);
      raylib.DrawCircle (Ball.X, Ball.Y, Ball_Fill, raylib.RAYWHITE);
      if Debug then
         raylib.DrawText (New_String (int'Image (Index)),
                          Ball.X,
                          Ball.Y,
                          25,
                          raylib.BLACK);
      end if;
   end Draw_Ball;

   procedure Draw_Game is
   begin
      raylib.BeginDrawing;
      raylib.ClearBackground (raylib.RAYWHITE);

      if Natural (Balls.Length) < 2 then
         raise Program_Error;
      end if;

      declare
         Color : raylib.Color;
         Index : int := 1;
      begin
         if Clear then
            Color := raylib.GREEN;
         else
            Color := raylib.BLACK;
         end if;
         for Ball_Index in 1 .. Natural (Balls.Length) - 1 loop
               raylib.DrawLine (Balls (Ball_Index).X,
                                Balls (Ball_Index).Y,
                                Balls (Ball_Index - 1).X,
                                Balls (Ball_Index - 1).Y,
                                Color);
         end loop;
         raylib.DrawLine (Balls (0).X,
                          Balls (0).Y,
                          Balls (Natural (Balls.Length) - 1).X,
                          Balls (Natural (Balls.Length) - 1).Y,
                          Color);

         for Ball of Balls loop
            Draw_Ball (Index, Ball);
            Index := Index + 1;
         end loop;
      end;

      raylib.EndDrawing;
   end Draw_Game;

begin

   Reset (Gx);
   Reset (Gy);

   for Not_Used in 1 .. Starting_Balls loop
      Balls.Append (Ball_Type'(X => Random (Gx), Y => Random (Gy)));
   end loop;

   New_Level;

   raylib.InitWindow (Game_Width, Game_Height, New_String ("Test Raylib - Ada"));
   raylib.InitAudioDevice;
   raylib.SetMasterVolume (0.3);

   if raylib.IsAudioDeviceReady then
      Put_Line ("Audio device ready!");
   else
      Put_Line ("Audio device NOT ready!");
   end if;

   Level_Complete_Wave := raylib.LoadWave (New_String ("level_complete.wav"));
   Level_Complete_Sound := raylib.LoadSoundFromWave (Level_Complete_Wave);

   raylib.SetTargetFPS (100);

   loop
      Update_Game;
      Draw_Game;
      exit when raylib.WindowShouldClose;
   end loop;
end Tangled;
