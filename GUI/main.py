import pygame
import pygame_gui
from pygame_gui.core import ObjectID
import chess.svg
from cairosvg import svg2png

pygame.init()

# Set up the window
width = 720
height = 480
window_size = (width, height)
screen = pygame.display.set_mode(window_size)

# Set up the UI manager
manager = pygame_gui.UIManager(window_size, "theme.json")
#manager.get_theme().load_theme("button.json")
background = pygame.Surface(window_size)

# Set up buttons for selecting game mode
start_button = pygame_gui.elements.UIButton(
    relative_rect=pygame.Rect((0, -100), (width, 100)),
    text='Start',
    manager=manager,
    anchors={'bottom': 'bottom',
             'right': 'right',
             'left': 'left'},
            object_id=ObjectID(class_id='@small_buttons')
)

logo = pygame.image.load("Icons/logo.png")

# Set up relevant info to send to master program
gamemode = ""
colour = ""
engine = ""
opponentID = ""
file = open("lichessToken.txt", "r")
lichessToken = file.read()
image = None

#set up buttons and stuff
player_vs_player_button = None
player_vs_engine_button = None
maia_button = None
viridithas_button = None
white_button = None
black_button = None
random_button = None
begin_button = None
cancel_button = None
cancel_button_player = None
cancel_button_engine = None
confirmID_button = None
wrongID_label = None
local_button = None
confirmSettings_button = None
settings_button = None


# Set up the clock for managing time
clock = pygame.time.Clock()

while True:
    # Get the time since the last loop iteration
    time_delta = clock.tick(60) / 1000.0

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            exit()

        # Check for button presses
        if event.type == pygame_gui.UI_BUTTON_PRESSED:
            if event.ui_element == start_button:
                logo = None
                start_button.kill()
                start_button = None
                player_vs_player_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/2, height/2)),
                    text='Player vs Player',
                    manager=manager,
                    anchors={'left': 'left',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pvp_button')
                )
                player_vs_engine_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, 0), (width/2, height/2)),
                    text='Player vs Engine',
                    manager=manager,
                    anchors={'right': 'right',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pve_button')
                )
                local_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -height/2), (width/2, height/2)),
                    text='Local game',
                    manager=manager,
                    anchors={'left': 'left',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#local_button')
                )
                settings_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, -height/2), (width/2, height/2)),
                    text='Settings',
                    manager=manager,
                    anchors={'right': 'right',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#settings_button')
                )
            elif event.ui_element == player_vs_player_button:
                # Handle player vs engine mode
                player_vs_player_button.kill()
                player_vs_player_button = None
                player_vs_engine_button.kill()
                player_vs_engine_button = None
                local_button.kill()
                local_button = None
                settings_button.kill()
                settings_button = None
                gamemode = "lichess"
                OpponentID_EntryLine = pygame_gui.elements.UITextEntryLine(
                    relative_rect=pygame.Rect((0, -50), (width -100, 100)),
                    placeholder_text="Opponent ID",
                    manager=manager,
                    anchors={'center': 'center'}
                )
                confirmID_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -180), (width, 100)),
                    text='Confirm',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
                cancel_button_player = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Return',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == cancel_button_player:
                confirmID_button.kill()
                confirmID_button = None
                OpponentID_EntryLine.kill()
                OpponentID_EntryLine = None
                cancel_button_player.kill()
                cancel_button_player = None
                if wrongID_label != None:
                    wrongID_label.kill()
                    wrongID_label = None
                player_vs_player_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/2, height/2)),
                    text='Player vs Player',
                    manager=manager,
                    anchors={'left': 'left',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pvp_button')
                )
                player_vs_engine_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, 0), (width/2, height/2)),
                    text='Player vs Engine',
                    manager=manager,
                    anchors={'right': 'right',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pve_button')
                )
                local_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -height/2), (width/2, height/2)),
                    text='Local game',
                    manager=manager,
                    anchors={'left': 'left',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#local_button')
                )
                settings_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, -height/2), (width/2, height/2)),
                    text='Settings',
                    manager=manager,
                    anchors={'right': 'right',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#settings_button')
                )
            elif event.ui_element == confirmID_button:
                if OpponentID_EntryLine.get_text() == "":
                    wrongID_label = pygame_gui.elements.UILabel(
                    relative_rect=pygame.Rect((0, 50), (300, 100)),
                    text="Please enter a valid opponent ID",
                    manager=manager,
                    anchors={'center': 'center'}
                )
                else:
                    opponentID = OpponentID_EntryLine.get_text()
                    OpponentID_EntryLine.kill()
                    OpponentID_EntryLine = None
                    confirmID_button.kill()
                    confirmID_button = None
                    cancel_button_player.kill()
                    cancel_button_player = None
                    if(wrongID_label != None):
                        wrongID_label.kill()
                        wrongID_label = None
                    white_button = pygame_gui.elements.UIButton(
                        relative_rect=pygame.Rect((0, 0), (width/3, height)),
                        text='White',
                        manager=manager,
                        anchors={'left': 'left',
                                'top': 'top',
                                'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#white_button')
                    )
                    black_button = pygame_gui.elements.UIButton(
                        relative_rect=pygame.Rect((-width/3,0), (width/3, height)),
                        text='Black',
                        manager=manager,
                        anchors={'right': 'right',
                                'top': 'top',
                                'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#black_button')
                    )
                    random_button = pygame_gui.elements.UIButton(
                        relative_rect=pygame.Rect((0, 0), (width/3, height)),
                        text='Random',
                        manager=manager,
                        anchors={'center': 'center',
                                'top': 'top',
                                'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#random_button')
                    )
            elif event.ui_element == player_vs_engine_button:
                # Handle player vs engine mode
                player_vs_player_button.kill()
                player_vs_player_button = None
                player_vs_engine_button.kill()
                player_vs_engine_button = None
                local_button.kill()
                local_button = None
                settings_button.kill()
                settings_button = None
                gamemode = "engine"
                viridithas_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/2, height-100)),
                    text='Viridithas',
                    manager=manager,
                    anchors={'left': 'left',
                             'top': 'top',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#viridithas_button')
                )
                maia_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, 0), (width/2, height-100)),
                    text='Maia',
                    manager=manager,
                    anchors={'right': 'right',
                             'top': 'top',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#maia_button')
                )
                cancel_button_engine = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Return',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == cancel_button_engine:
                viridithas_button.kill()
                viridithas_button = None
                maia_button.kill()
                maia_button = None
                cancel_button_engine.kill()
                cancel_button_engine = None
                player_vs_player_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/2, height/2)),
                    text='Player vs Player',
                    manager=manager,
                    anchors={'left': 'left',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pvp_button')
                )
                player_vs_engine_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, 0), (width/2, height/2)),
                    text='Player vs Engine',
                    manager=manager,
                    anchors={'right': 'right',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pve_button')
                )
                local_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -height/2), (width/2, height/2)),
                    text='Local game',
                    manager=manager,
                    anchors={'left': 'left',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#local_button')
                )
                settings_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, -height/2), (width/2, height/2)),
                    text='Settings',
                    manager=manager,
                    anchors={'right': 'right',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#settings_button')
                )
            elif event.ui_element == viridithas_button:
                viridithas_button.kill()
                viridithas_button = None
                maia_button.kill()
                maia_button = None
                cancel_button_engine.kill()
                cancel_button_engine = None
                engine = "v"
                white_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/3, height)),
                    text='White',
                    manager=manager,
                    anchors={'left': 'left',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#white_button')
                )
                black_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/3,0), (width/3, height)),
                    text='Black',
                    manager=manager,
                    anchors={'right': 'right',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#black_button')
                )
                random_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/3, height)),
                    text='Random',
                    manager=manager,
                    anchors={'center': 'center',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#random_button')
                )
            elif event.ui_element == maia_button:
                viridithas_button.kill()
                viridithas_button = None
                maia_button.kill()
                maia_button = None
                cancel_button_engine.kill()
                cancel_button_engine = None
                engine = "m"
                white_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/3, height)),
                    text='White',
                    manager=manager,
                    anchors={'left': 'left',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#white_button')
                )
                black_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/3,0), (width/3, height)),
                    text='Black',
                    manager=manager,
                    anchors={'right': 'right',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#black_button')
                )
                random_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/3, height)),
                    text='Random',
                    manager=manager,
                    anchors={'center': 'center',
                            'top': 'top',
                            'bottom': 'bottom'},
                object_id= ObjectID(object_id='#random_button')
                )
            elif event.ui_element == white_button:
                white_button.kill()
                white_button = None
                black_button.kill()
                black_button = None
                random_button.kill()
                random_button = None
                colour = "white"
                begin_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width, height-100)),
                    text='',
                    manager=manager,
                    anchors={'top': 'top',
                             'right': 'right',
                             'left': 'left'},
                object_id= ObjectID(object_id='#begin_button')
                )
                cancel_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Cancel',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == black_button:
                white_button.kill()
                white_button = None
                black_button.kill()
                black_button = None
                random_button.kill()
                random_button = None
                colour = "black"
                begin_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width, height-100)),
                    text='',
                    manager=manager,
                    anchors={'top': 'top',
                             'right': 'right',
                             'left': 'left'},
                object_id= ObjectID(object_id='#begin_button')
                )
                cancel_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Cancel',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == random_button:
                white_button.kill()
                white_button = None
                black_button.kill()
                black_button = None
                random_button.kill()
                random_button = None
                colour = "random"
                begin_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width, height-100)),
                    text='',
                    manager=manager,
                    anchors={'top': 'top',
                             'right': 'right',
                             'left': 'left'},
                object_id= ObjectID(object_id='#begin_button')
                )
                cancel_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Cancel',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == local_button:
                player_vs_player_button.kill()
                player_vs_player_button = None
                player_vs_engine_button.kill()
                player_vs_engine_button = None
                local_button.kill()
                local_button = None
                settings_button.kill()
                settings_button = None
                gamemode = "local"
                begin_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width, height-100)),
                    text='',
                    manager=manager,
                    anchors={'top': 'top',
                             'right': 'right',
                             'left': 'left'},
                object_id= ObjectID(object_id='#begin_button')
                )
                cancel_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Cancel',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == settings_button:
                player_vs_player_button.kill()
                player_vs_player_button = None
                player_vs_engine_button.kill()
                player_vs_engine_button = None
                local_button.kill()
                local_button = None
                settings_button.kill()
                settings_button = None
                Token_EntryLine = pygame_gui.elements.UITextEntryLine(
                    relative_rect=pygame.Rect((0, -50), (width -100, 100)),
                    placeholder_text=lichessToken,
                    manager=manager,
                    anchors={'center': 'center'}
                )
                Token_EntryLine.text = lichessToken
                confirmSettings_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -180), (width, 100)),
                    text='Confirm',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
            elif event.ui_element == confirmSettings_button:
                lichessToken = Token_EntryLine.get_text()
                file = open("lichessToken.txt", "w")
                file.write(lichessToken)
                file.close()
                Token_EntryLine.kill()
                Token_EntryLine = None
                confirmSettings_button.kill()
                confirmSettings_button = None
                player_vs_player_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, 0), (width/2, height/2)),
                    text='Player vs Player',
                    manager=manager,
                    anchors={'left': 'left',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pvp_button')
                )
                player_vs_engine_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, 0), (width/2, height/2)),
                    text='Player vs Engine',
                    manager=manager,
                    anchors={'right': 'right',
                             'top': 'top'},
                    object_id= ObjectID(object_id='#pve_button')
                )
                local_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -height/2), (width/2, height/2)),
                    text='Local game',
                    manager=manager,
                    anchors={'left': 'left',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#local_button')
                )
                settings_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((-width/2, -height/2), (width/2, height/2)),
                    text='Settings',
                    manager=manager,
                    anchors={'right': 'right',
                             'bottom': 'bottom'},
                    object_id= ObjectID(object_id='#settings_button')
                )
            elif event.ui_element == begin_button:
                begin_button.kill()
                begin_button = None
                cancel_button.kill()
                cancel_button = None
                gameInProgress_label = pygame_gui.elements.UILabel(
                    relative_rect=pygame.Rect((0, 0), (300, 100)),
                    text="Game in Progress",
                    manager=manager,
                    anchors={'center': 'center'}
                )
                #board = chess.Board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                #svg = chess.svg.board(
                #    board
                #)

                #svg2png(bytestring=svg,write_to='temp.png')
                #image = pygame.image.load("temp.png")
                print(gamemode)
                print(engine)
                print(colour)
                print(opponentID)
                print(lichessToken)
            elif event.ui_element == cancel_button:
                begin_button.kill()
                begin_button = None
                cancel_button.kill()
                cancel_button = None
                logo = pygame.image.load("Icons/logo.png")
                start_button = pygame_gui.elements.UIButton(
                    relative_rect=pygame.Rect((0, -100), (width, 100)),
                    text='Start',
                    manager=manager,
                    anchors={'bottom': 'bottom',
                             'right': 'right',
                             'left': 'left'},
                    object_id=ObjectID(class_id='@small_buttons')
                )
                gamemode = ""
                colour = ""
                engine = ""
                opponentID = ""

        # Process events for the UI manager
        manager.process_events(event)

    # Update the UI manager
    manager.update(time_delta)

    # Draw the UI

    screen.fill((255,198,108))
    if(logo != None): pygame.Surface.blit(screen,logo,(width/2-222, 50))
    manager.draw_ui(screen)
    pygame.display.update()